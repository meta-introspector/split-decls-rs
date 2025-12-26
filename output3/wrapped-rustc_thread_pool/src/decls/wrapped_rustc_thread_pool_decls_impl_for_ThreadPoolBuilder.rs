use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<S> ThreadPoolBuilder<S> {
    /// Sets a custom function for spawning threads.
    ///
    /// Note that the threads will not exit until after the pool is dropped. It
    /// is up to the caller to wait for thread termination if that is important
    /// for any invariants. For instance, threads created in [`std::thread::scope`]
    /// will be joined before that scope returns, and this will block indefinitely
    /// if the pool is leaked. Furthermore, the global thread pool doesn't terminate
    /// until the entire process exits!
    ///
    /// # Examples
    ///
    /// A minimal spawn handler just needs to call `run()` from an independent thread.
    ///
    /// ```
    /// # use rustc_thread_pool as rayon;
    /// fn main() -> Result<(), rayon::ThreadPoolBuildError> {
    ///     let pool = rayon::ThreadPoolBuilder::new()
    ///         .spawn_handler(|thread| {
    ///             std::thread::spawn(|| thread.run());
    ///             Ok(())
    ///         })
    ///         .build()?;
    ///
    ///     pool.install(|| println!("Hello from my custom thread!"));
    ///     Ok(())
    /// }
    /// ```
    ///
    /// The default spawn handler sets the name and stack size if given, and propagates
    /// any errors from the thread builder.
    ///
    /// ```
    /// # use rustc_thread_pool as rayon;
    /// fn main() -> Result<(), rayon::ThreadPoolBuildError> {
    ///     let pool = rayon::ThreadPoolBuilder::new()
    ///         .spawn_handler(|thread| {
    ///             let mut b = std::thread::Builder::new();
    ///             if let Some(name) = thread.name() {
    ///                 b = b.name(name.to_owned());
    ///             }
    ///             if let Some(stack_size) = thread.stack_size() {
    ///                 b = b.stack_size(stack_size);
    ///             }
    ///             b.spawn(|| thread.run())?;
    ///             Ok(())
    ///         })
    ///         .build()?;
    ///
    ///     pool.install(|| println!("Hello from my fully custom thread!"));
    ///     Ok(())
    /// }
    /// ```
    ///
    /// This can also be used for a pool of scoped threads like [`crossbeam::scope`],
    /// or [`std::thread::scope`] introduced in Rust 1.63, which is encapsulated in
    /// [`build_scoped`](#method.build_scoped).
    ///
    /// [`crossbeam::scope`]: https://docs.rs/crossbeam/0.8/crossbeam/fn.scope.html
    /// [`std::thread::scope`]: https://doc.rust-lang.org/std/thread/fn.scope.html
    ///
    /// ```
    /// # use rustc_thread_pool as rayon;
    /// fn main() -> Result<(), rayon::ThreadPoolBuildError> {
    ///     std::thread::scope(|scope| {
    ///         let pool = rayon::ThreadPoolBuilder::new()
    ///             .spawn_handler(|thread| {
    ///                 let mut builder = std::thread::Builder::new();
    ///                 if let Some(name) = thread.name() {
    ///                     builder = builder.name(name.to_string());
    ///                 }
    ///                 if let Some(size) = thread.stack_size() {
    ///                     builder = builder.stack_size(size);
    ///                 }
    ///                 builder.spawn_scoped(scope, || {
    ///                     // Add any scoped initialization here, then run!
    ///                     thread.run()
    ///                 })?;
    ///                 Ok(())
    ///             })
    ///             .build()?;
    ///
    ///         pool.install(|| println!("Hello from my custom scoped thread!"));
    ///         Ok(())
    ///     })
    /// }
    /// ```
    pub fn spawn_handler<F>(self, spawn: F) -> ThreadPoolBuilder<CustomSpawn<F>>
    where
        F: FnMut(ThreadBuilder) -> io::Result<()>,
    {
        ThreadPoolBuilder {
            spawn_handler: CustomSpawn::new(spawn),
            num_threads: self.num_threads,
            panic_handler: self.panic_handler,
            get_thread_name: self.get_thread_name,
            stack_size: self.stack_size,
            start_handler: self.start_handler,
            exit_handler: self.exit_handler,
            deadlock_handler: self.deadlock_handler,
            acquire_thread_handler: self.acquire_thread_handler,
            release_thread_handler: self.release_thread_handler,
            breadth_first: self.breadth_first,
        }
    }
    /// Returns a reference to the current spawn handler.
    fn get_spawn_handler(&mut self) -> &mut S {
        &mut self.spawn_handler
    }
    /// Get the number of threads that will be used for the thread
    /// pool. See `num_threads()` for more information.
    fn get_num_threads(&self) -> usize {
        if self.num_threads > 0 {
            self.num_threads
        } else {
            let default = || {
                thread::available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(1)
            };
            match env::var("RAYON_NUM_THREADS")
                .ok()
                .and_then(|s| usize::from_str(&s).ok())
            {
                Some(x @ 1..) => return x,
                Some(0) => return default(),
                _ => {}
            }
            match env::var("RAYON_RS_NUM_CPUS")
                .ok()
                .and_then(|s| usize::from_str(&s).ok())
            {
                Some(x @ 1..) => x,
                _ => default(),
            }
        }
    }
    /// Get the thread name for the thread with the given index.
    fn get_thread_name(&mut self, index: usize) -> Option<String> {
        let f = self.get_thread_name.as_mut()?;
        Some(f(index))
    }
    /// Sets a closure which takes a thread index and returns
    /// the thread's name.
    pub fn thread_name<F>(mut self, closure: F) -> Self
    where
        F: FnMut(usize) -> String + 'static,
    {
        self.get_thread_name = Some(Box::new(closure));
        self
    }
    /// Sets the number of threads to be used in the rayon threadpool.
    ///
    /// If you specify a non-zero number of threads using this
    /// function, then the resulting thread-pools are guaranteed to
    /// start at most this number of threads.
    ///
    /// If `num_threads` is 0, or you do not call this function, then
    /// the Rayon runtime will select the number of threads
    /// automatically. At present, this is based on the
    /// `RAYON_NUM_THREADS` environment variable (if set),
    /// or the number of logical CPUs (otherwise).
    /// In the future, however, the default behavior may
    /// change to dynamically add or remove threads as needed.
    ///
    /// **Future compatibility warning:** Given the default behavior
    /// may change in the future, if you wish to rely on a fixed
    /// number of threads, you should use this function to specify
    /// that number. To reproduce the current default behavior, you
    /// may wish to use [`std::thread::available_parallelism`]
    /// to query the number of CPUs dynamically.
    ///
    /// **Old environment variable:** `RAYON_NUM_THREADS` is a one-to-one
    /// replacement of the now deprecated `RAYON_RS_NUM_CPUS` environment
    /// variable. If both variables are specified, `RAYON_NUM_THREADS` will
    /// be preferred.
    pub fn num_threads(mut self, num_threads: usize) -> Self {
        self.num_threads = num_threads;
        self
    }
    /// Returns a copy of the current panic handler.
    fn take_panic_handler(&mut self) -> Option<Box<PanicHandler>> {
        self.panic_handler.take()
    }
    /// Normally, whenever Rayon catches a panic, it tries to
    /// propagate it to someplace sensible, to try and reflect the
    /// semantics of sequential execution. But in some cases,
    /// particularly with the `spawn()` APIs, there is no
    /// obvious place where we should propagate the panic to.
    /// In that case, this panic handler is invoked.
    ///
    /// If no panic handler is set, the default is to abort the
    /// process, under the principle that panics should not go
    /// unobserved.
    ///
    /// If the panic handler itself panics, this will abort the
    /// process. To prevent this, wrap the body of your panic handler
    /// in a call to `std::panic::catch_unwind()`.
    pub fn panic_handler<H>(mut self, panic_handler: H) -> Self
    where
        H: Fn(Box<dyn Any + Send>) + Send + Sync + 'static,
    {
        self.panic_handler = Some(Box::new(panic_handler));
        self
    }
    /// Get the stack size of the worker threads
    fn get_stack_size(&self) -> Option<usize> {
        self.stack_size
    }
    /// Sets the stack size of the worker threads
    pub fn stack_size(mut self, stack_size: usize) -> Self {
        self.stack_size = Some(stack_size);
        self
    }
    /// **(DEPRECATED)** Suggest to worker threads that they execute
    /// spawned jobs in a "breadth-first" fashion.
    ///
    /// Typically, when a worker thread is idle or blocked, it will
    /// attempt to execute the job from the *top* of its local deque of
    /// work (i.e., the job most recently spawned). If this flag is set
    /// to true, however, workers will prefer to execute in a
    /// *breadth-first* fashion -- that is, they will search for jobs at
    /// the *bottom* of their local deque. (At present, workers *always*
    /// steal from the bottom of other workers' deques, regardless of
    /// the setting of this flag.)
    ///
    /// If you think of the tasks as a tree, where a parent task
    /// spawns its children in the tree, then this flag loosely
    /// corresponds to doing a breadth-first traversal of the tree,
    /// whereas the default would be to do a depth-first traversal.
    ///
    /// **Note that this is an "execution hint".** Rayon's task
    /// execution is highly dynamic and the precise order in which
    /// independent tasks are executed is not intended to be
    /// guaranteed.
    ///
    /// This `breadth_first()` method is now deprecated per [RFC #1],
    /// and in the future its effect may be removed. Consider using
    /// [`scope_fifo()`] for a similar effect.
    ///
    /// [RFC #1]: https://github.com/rayon-rs/rfcs/blob/master/accepted/rfc0001-scope-scheduling.md
    /// [`scope_fifo()`]: fn.scope_fifo.html
    #[deprecated(note = "use `scope_fifo` and `spawn_fifo` for similar effect")]
    pub fn breadth_first(mut self) -> Self {
        self.breadth_first = true;
        self
    }
    fn get_breadth_first(&self) -> bool {
        self.breadth_first
    }
    /// Takes the current acquire thread callback, leaving `None`.
    fn take_acquire_thread_handler(&mut self) -> Option<Box<AcquireThreadHandler>> {
        self.acquire_thread_handler.take()
    }
    /// Set a callback to be invoked when starting computations in a thread.
    pub fn acquire_thread_handler<H>(mut self, acquire_thread_handler: H) -> Self
    where
        H: Fn() + Send + Sync + 'static,
    {
        self.acquire_thread_handler = Some(Box::new(acquire_thread_handler));
        self
    }
    /// Takes the current release thread callback, leaving `None`.
    fn take_release_thread_handler(&mut self) -> Option<Box<ReleaseThreadHandler>> {
        self.release_thread_handler.take()
    }
    /// Set a callback to be invoked when blocking in thread.
    pub fn release_thread_handler<H>(mut self, release_thread_handler: H) -> Self
    where
        H: Fn() + Send + Sync + 'static,
    {
        self.release_thread_handler = Some(Box::new(release_thread_handler));
        self
    }
    /// Takes the current deadlock callback, leaving `None`.
    fn take_deadlock_handler(&mut self) -> Option<Box<DeadlockHandler>> {
        self.deadlock_handler.take()
    }
    /// Set a callback to be invoked on current deadlock.
    pub fn deadlock_handler<H>(mut self, deadlock_handler: H) -> Self
    where
        H: Fn() + Send + Sync + 'static,
    {
        self.deadlock_handler = Some(Box::new(deadlock_handler));
        self
    }
    /// Takes the current thread start callback, leaving `None`.
    fn take_start_handler(&mut self) -> Option<Box<StartHandler>> {
        self.start_handler.take()
    }
    /// Sets a callback to be invoked on thread start.
    ///
    /// The closure is passed the index of the thread on which it is invoked.
    /// Note that this same closure may be invoked multiple times in parallel.
    /// If this closure panics, the panic will be passed to the panic handler.
    /// If that handler returns, then startup will continue normally.
    pub fn start_handler<H>(mut self, start_handler: H) -> Self
    where
        H: Fn(usize) + Send + Sync + 'static,
    {
        self.start_handler = Some(Box::new(start_handler));
        self
    }
    /// Returns a current thread exit callback, leaving `None`.
    fn take_exit_handler(&mut self) -> Option<Box<ExitHandler>> {
        self.exit_handler.take()
    }
    /// Sets a callback to be invoked on thread exit.
    ///
    /// The closure is passed the index of the thread on which it is invoked.
    /// Note that this same closure may be invoked multiple times in parallel.
    /// If this closure panics, the panic will be passed to the panic handler.
    /// If that handler returns, then the thread will exit normally.
    pub fn exit_handler<H>(mut self, exit_handler: H) -> Self
    where
        H: Fn(usize) + Send + Sync + 'static,
    {
        self.exit_handler = Some(Box::new(exit_handler));
        self
    }
}
