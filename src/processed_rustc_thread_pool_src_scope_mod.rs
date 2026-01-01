/* FP:mod.rs-0001 */ // Methods for custom fork-join scopes, created by the [`scope()`]
/* FP:mod.rs-0002 */ // and [`in_place_scope()`] functions. These are a more flexible alternative to [`join()`].
/* FP:mod.rs-0003 */ //
/* FP:mod.rs-0004 */ // [`scope()`]: fn.scope.html
/* FP:mod.rs-0005 */ // [`in_place_scope()`]: fn.in_place_scope.html
/* FP:mod.rs-0006 */ // [`join()`]: ../join/join.fn.html
/* FP:mod.rs-0007 */ 
/* FP:mod.rs-0008 */ use std::any::Any;
/* FP:mod.rs-0009 */ use std::collections::HashSet;
/* FP:mod.rs-0010 */ use std::marker::PhantomData;
/* FP:mod.rs-0011 */ use std::mem::ManuallyDrop;
/* FP:mod.rs-0012 */ use std::sync::atomic::{AtomicPtr, Ordering};
/* FP:mod.rs-0013 */ use std::sync::{Arc, Mutex};
/* FP:mod.rs-0014 */ use std::{fmt, ptr};
/* FP:mod.rs-0015 */ 
/* FP:mod.rs-0016 */ use crate::broadcast::BroadcastContext;
/* FP:mod.rs-0017 */ use crate::job::{ArcJob, HeapJob, JobFifo, JobRef, JobRefId};
/* FP:mod.rs-0018 */ use crate::latch::{CountLatch, Latch};
/* FP:mod.rs-0019 */ use crate::registry::{Registry, WorkerThread, global_registry, in_worker};
/* FP:mod.rs-0020 */ use crate::tlv::{self, Tlv};
/* FP:mod.rs-0021 */ use crate::unwind;
/* FP:mod.rs-0022 */ 
/* FP:mod.rs-0023 */ #[cfg(test)]
/* FP:mod.rs-0025 */ 
/* FP:mod.rs-0026 */ /// Represents a fork-join scope which can be used to spawn any number of tasks.
/* FP:mod.rs-0027 */ /// See [`scope()`] for more information.
/* FP:mod.rs-0028 */ ///
/* FP:mod.rs-0029 */ ///[`scope()`]: fn.scope.html
/* FP:mod.rs-0030 */ pub struct Scope<'scope> {
/* FP:mod.rs-0031 */     base: ScopeBase<'scope>,
/* FP:mod.rs-0032 */ }
/* FP:mod.rs-0033 */ 
/* FP:mod.rs-0034 */ /// Represents a fork-join scope which can be used to spawn any number of tasks.
/* FP:mod.rs-0035 */ /// Those spawned from the same thread are prioritized in relative FIFO order.
/* FP:mod.rs-0036 */ /// See [`scope_fifo()`] for more information.
/* FP:mod.rs-0037 */ ///
/* FP:mod.rs-0038 */ ///[`scope_fifo()`]: fn.scope_fifo.html
/* FP:mod.rs-0039 */ pub struct ScopeFifo<'scope> {
/* FP:mod.rs-0040 */     base: ScopeBase<'scope>,
/* FP:mod.rs-0041 */     fifos: Vec<JobFifo>,
/* FP:mod.rs-0042 */ }
/* FP:mod.rs-0043 */ 
/* FP:mod.rs-0044 */ struct ScopeBase<'scope> {
/* FP:mod.rs-0045 */     /// thread registry where `scope()` was executed or where `in_place_scope()`
/* FP:mod.rs-0046 */     /// should spawn jobs.
/* FP:mod.rs-0047 */     registry: Arc<Registry>,
/* FP:mod.rs-0048 */ 
/* FP:mod.rs-0049 */     /// if some job panicked, the error is stored here; it will be
/* FP:mod.rs-0050 */     /// propagated to the one who created the scope
/* FP:mod.rs-0051 */     panic: AtomicPtr<Box<dyn Any + Send + 'static>>,
/* FP:mod.rs-0052 */ 
/* FP:mod.rs-0053 */     /// latch to track job counts
/* FP:mod.rs-0054 */     job_completed_latch: CountLatch,
/* FP:mod.rs-0055 */ 
/* FP:mod.rs-0056 */     /// Jobs that have been spawned, but not yet started.
/* FP:mod.rs-0057 */     #[allow(rustc::default_hash_types)]
/* FP:mod.rs-0058 */     pending_jobs: Mutex<HashSet<JobRefId>>,
/* FP:mod.rs-0059 */ 
/* FP:mod.rs-0060 */     /// The worker which will wait on scope completion, if any.
/* FP:mod.rs-0061 */     worker: Option<usize>,
/* FP:mod.rs-0062 */ 
/* FP:mod.rs-0063 */     /// You can think of a scope as containing a list of closures to execute,
/* FP:mod.rs-0064 */     /// all of which outlive `'scope`. They're not actually required to be
/* FP:mod.rs-0065 */     /// `Sync`, but it's still safe to let the `Scope` implement `Sync` because
/* FP:mod.rs-0066 */     /// the closures are only *moved* across threads to be executed.
/* FP:mod.rs-0067 */     #[allow(clippy::type_complexity)]
/* FP:mod.rs-0068 */     marker: PhantomData<Box<dyn FnOnce(&Scope<'scope>) + Send + Sync + 'scope>>,
/* FP:mod.rs-0069 */ 
/* FP:mod.rs-0070 */     /// The TLV at the scope's creation. Used to set the TLV for spawned jobs.
/* FP:mod.rs-0071 */     tlv: Tlv,
/* FP:mod.rs-0072 */ }
/* FP:mod.rs-0073 */ 
/* FP:mod.rs-0074 */ /// Creates a "fork-join" scope `s` and invokes the closure with a
/* FP:mod.rs-0075 */ /// reference to `s`. This closure can then spawn asynchronous tasks
/* FP:mod.rs-0076 */ /// into `s`. Those tasks may run asynchronously with respect to the
/* FP:mod.rs-0077 */ /// closure; they may themselves spawn additional tasks into `s`. When
/* FP:mod.rs-0078 */ /// the closure returns, it will block until all tasks that have been
/* FP:mod.rs-0079 */ /// spawned into `s` complete.
/* FP:mod.rs-0080 */ ///
/* FP:mod.rs-0081 */ /// `scope()` is a more flexible building block compared to `join()`,
/* FP:mod.rs-0082 */ /// since a loop can be used to spawn any number of tasks without
/* FP:mod.rs-0083 */ /// recursing. However, that flexibility comes at a performance price:
/* FP:mod.rs-0084 */ /// tasks spawned using `scope()` must be allocated onto the heap,
/* FP:mod.rs-0085 */ /// whereas `join()` can make exclusive use of the stack. **Prefer
/* FP:mod.rs-0086 */ /// `join()` (or, even better, parallel iterators) where possible.**
/* FP:mod.rs-0087 */ ///
/* FP:mod.rs-0088 */ /// # Example
/* FP:mod.rs-0089 */ ///
/* FP:mod.rs-0090 */ /// The Rayon `join()` function launches two closures and waits for them
/* FP:mod.rs-0091 */ /// to stop. One could implement `join()` using a scope like so, although
/* FP:mod.rs-0092 */ /// it would be less efficient than the real implementation:
/* FP:mod.rs-0093 */ ///
/* FP:mod.rs-0094 */ /// ```rust
/* FP:mod.rs-0095 */ /// # use rustc_thread_pool as rayon;
/* FP:mod.rs-0096 */ /// pub fn join<A,B,RA,RB>(oper_a: A, oper_b: B) -> (RA, RB)
/* FP:mod.rs-0097 */ ///     where A: FnOnce() -> RA + Send,
/* FP:mod.rs-0098 */ ///           B: FnOnce() -> RB + Send,
/* FP:mod.rs-0099 */ ///           RA: Send,
/* FP:mod.rs-0100 */ ///           RB: Send,
/* FP:mod.rs-0101 */ /// {
/* FP:mod.rs-0102 */ ///     let mut result_a: Option<RA> = None;
/* FP:mod.rs-0103 */ ///     let mut result_b: Option<RB> = None;
/* FP:mod.rs-0104 */ ///     rayon::scope(|s| {
/* FP:mod.rs-0105 */ ///         s.spawn(|_| result_a = Some(oper_a()));
/* FP:mod.rs-0106 */ ///         s.spawn(|_| result_b = Some(oper_b()));
/* FP:mod.rs-0107 */ ///     });
/* FP:mod.rs-0108 */ ///     (result_a.unwrap(), result_b.unwrap())
/* FP:mod.rs-0109 */ /// }
/* FP:mod.rs-0110 */ /// ```
/* FP:mod.rs-0111 */ ///
/* FP:mod.rs-0112 */ /// # A note on threading
/* FP:mod.rs-0113 */ ///
/* FP:mod.rs-0114 */ /// The closure given to `scope()` executes in the Rayon thread-pool,
/* FP:mod.rs-0115 */ /// as do those given to `spawn()`. This means that you can't access
/* FP:mod.rs-0116 */ /// thread-local variables (well, you can, but they may have
/* FP:mod.rs-0117 */ /// unexpected values).
/* FP:mod.rs-0118 */ ///
/* FP:mod.rs-0119 */ /// # Task execution
/* FP:mod.rs-0120 */ ///
/* FP:mod.rs-0121 */ /// Task execution potentially starts as soon as `spawn()` is called.
/* FP:mod.rs-0122 */ /// The task will end sometime before `scope()` returns. Note that the
/* FP:mod.rs-0123 */ /// *closure* given to scope may return much earlier. In general
/* FP:mod.rs-0124 */ /// the lifetime of a scope created like `scope(body)` goes something like this:
/* FP:mod.rs-0125 */ ///
/* FP:mod.rs-0126 */ /// - Scope begins when `scope(body)` is called
/* FP:mod.rs-0127 */ /// - Scope body `body()` is invoked
/* FP:mod.rs-0128 */ ///     - Scope tasks may be spawned
/* FP:mod.rs-0129 */ /// - Scope body returns
/* FP:mod.rs-0130 */ /// - Scope tasks execute, possibly spawning more tasks
/* FP:mod.rs-0131 */ /// - Once all tasks are done, scope ends and `scope()` returns
/* FP:mod.rs-0132 */ ///
/* FP:mod.rs-0133 */ /// To see how and when tasks are joined, consider this example:
/* FP:mod.rs-0134 */ ///
/* FP:mod.rs-0135 */ /// ```rust
/* FP:mod.rs-0136 */ /// # use rustc_thread_pool as rayon;
/* FP:mod.rs-0137 */ /// // point start
/* FP:mod.rs-0138 */ /// rayon::scope(|s| {
/* FP:mod.rs-0139 */ ///     s.spawn(|s| { // task s.1
/* FP:mod.rs-0140 */ ///         s.spawn(|s| { // task s.1.1
/* FP:mod.rs-0141 */ ///             rayon::scope(|t| {
/* FP:mod.rs-0142 */ ///                 t.spawn(|_| ()); // task t.1
/* FP:mod.rs-0143 */ ///                 t.spawn(|_| ()); // task t.2
/* FP:mod.rs-0144 */ ///             });
/* FP:mod.rs-0145 */ ///         });
/* FP:mod.rs-0146 */ ///     });
/* FP:mod.rs-0147 */ ///     s.spawn(|s| { // task s.2
/* FP:mod.rs-0148 */ ///     });
/* FP:mod.rs-0149 */ ///     // point mid
/* FP:mod.rs-0150 */ /// });
/* FP:mod.rs-0151 */ /// // point end
/* FP:mod.rs-0152 */ /// ```
/* FP:mod.rs-0153 */ ///
/* FP:mod.rs-0154 */ /// The various tasks that are run will execute roughly like so:
/* FP:mod.rs-0155 */ ///
/* FP:mod.rs-0156 */ /// ```notrust
/* FP:mod.rs-0157 */ /// | (start)
/* FP:mod.rs-0158 */ /// |
/* FP:mod.rs-0159 */ /// | (scope `s` created)
/* FP:mod.rs-0160 */ /// +-----------------------------------------------+ (task s.2)
/* FP:mod.rs-0161 */ /// +-------+ (task s.1)                            |
/* FP:mod.rs-0162 */ /// |       |                                       |
/* FP:mod.rs-0163 */ /// |       +---+ (task s.1.1)                      |
/* FP:mod.rs-0164 */ /// |       |   |                                   |
/* FP:mod.rs-0165 */ /// |       |   | (scope `t` created)               |
/* FP:mod.rs-0166 */ /// |       |   +----------------+ (task t.2)       |
/* FP:mod.rs-0167 */ /// |       |   +---+ (task t.1) |                  |
/* FP:mod.rs-0168 */ /// | (mid) |   |   |            |                  |
/* FP:mod.rs-0169 */ /// :       |   + <-+------------+ (scope `t` ends) |
/* FP:mod.rs-0170 */ /// :       |   |                                   |
/* FP:mod.rs-0171 */ /// |<------+---+-----------------------------------+ (scope `s` ends)
/* FP:mod.rs-0172 */ /// |
/* FP:mod.rs-0173 */ /// | (end)
/* FP:mod.rs-0174 */ /// ```
/* FP:mod.rs-0175 */ ///
/* FP:mod.rs-0176 */ /// The point here is that everything spawned into scope `s` will
/* FP:mod.rs-0177 */ /// terminate (at latest) at the same point -- right before the
/* FP:mod.rs-0178 */ /// original call to `rayon::scope` returns. This includes new
/* FP:mod.rs-0179 */ /// subtasks created by other subtasks (e.g., task `s.1.1`). If a new
/* FP:mod.rs-0180 */ /// scope is created (such as `t`), the things spawned into that scope
/* FP:mod.rs-0181 */ /// will be joined before that scope returns, which in turn occurs
/* FP:mod.rs-0182 */ /// before the creating task (task `s.1.1` in this case) finishes.
/* FP:mod.rs-0183 */ ///
/* FP:mod.rs-0184 */ /// There is no guaranteed order of execution for spawns in a scope,
/* FP:mod.rs-0185 */ /// given that other threads may steal tasks at any time. However, they
/* FP:mod.rs-0186 */ /// are generally prioritized in a LIFO order on the thread from which
/* FP:mod.rs-0187 */ /// they were spawned. So in this example, absent any stealing, we can
/* FP:mod.rs-0188 */ /// expect `s.2` to execute before `s.1`, and `t.2` before `t.1`. Other
/* FP:mod.rs-0189 */ /// threads always steal from the other end of the deque, like FIFO
/* FP:mod.rs-0190 */ /// order. The idea is that "recent" tasks are most likely to be fresh
/* FP:mod.rs-0191 */ /// in the local CPU's cache, while other threads can steal older
/* FP:mod.rs-0192 */ /// "stale" tasks. For an alternate approach, consider
/* FP:mod.rs-0193 */ /// [`scope_fifo()`] instead.
/* FP:mod.rs-0194 */ ///
/* FP:mod.rs-0195 */ /// [`scope_fifo()`]: fn.scope_fifo.html
/* FP:mod.rs-0196 */ ///
/* FP:mod.rs-0197 */ /// # Accessing stack data
/* FP:mod.rs-0198 */ ///
/* FP:mod.rs-0199 */ /// In general, spawned tasks may access stack data in place that
/* FP:mod.rs-0200 */ /// outlives the scope itself. Other data must be fully owned by the
/* FP:mod.rs-0201 */ /// spawned task.
/* FP:mod.rs-0202 */ ///
/* FP:mod.rs-0203 */ /// ```rust
/* FP:mod.rs-0204 */ /// # use rustc_thread_pool as rayon;
/* FP:mod.rs-0205 */ /// let ok: Vec<i32> = vec![1, 2, 3];
/* FP:mod.rs-0206 */ /// rayon::scope(|s| {
/* FP:mod.rs-0207 */ ///     let bad: Vec<i32> = vec![4, 5, 6];
/* FP:mod.rs-0208 */ ///     s.spawn(|_| {
/* FP:mod.rs-0209 */ ///         // We can access `ok` because outlives the scope `s`.
/* FP:mod.rs-0210 */ ///         println!("ok: {:?}", ok);
/* FP:mod.rs-0211 */ ///
/* FP:mod.rs-0212 */ ///         // If we just try to use `bad` here, the closure will borrow `bad`
/* FP:mod.rs-0213 */ ///         // (because we are just printing it out, and that only requires a
/* FP:mod.rs-0214 */ ///         // borrow), which will result in a compilation error. Read on
/* FP:mod.rs-0215 */ ///         // for options.
/* FP:mod.rs-0216 */ ///         // println!("bad: {:?}", bad);
/* FP:mod.rs-0217 */ ///    });
/* FP:mod.rs-0218 */ /// });
/* FP:mod.rs-0219 */ /// ```
/* FP:mod.rs-0220 */ ///
/* FP:mod.rs-0221 */ /// As the comments example above suggest, to reference `bad` we must
/* FP:mod.rs-0222 */ /// take ownership of it. One way to do this is to detach the closure
/* FP:mod.rs-0223 */ /// from the surrounding stack frame, using the `move` keyword. This
/* FP:mod.rs-0224 */ /// will cause it to take ownership of *all* the variables it touches,
/* FP:mod.rs-0225 */ /// in this case including both `ok` *and* `bad`:
/* FP:mod.rs-0226 */ ///
/* FP:mod.rs-0227 */ /// ```rust
/* FP:mod.rs-0228 */ /// # use rustc_thread_pool as rayon;
/* FP:mod.rs-0229 */ /// let ok: Vec<i32> = vec![1, 2, 3];
/* FP:mod.rs-0230 */ /// rayon::scope(|s| {
/* FP:mod.rs-0231 */ ///     let bad: Vec<i32> = vec![4, 5, 6];
/* FP:mod.rs-0232 */ ///     s.spawn(move |_| {
/* FP:mod.rs-0233 */ ///         println!("ok: {:?}", ok);
/* FP:mod.rs-0234 */ ///         println!("bad: {:?}", bad);
/* FP:mod.rs-0235 */ ///     });
/* FP:mod.rs-0236 */ ///
/* FP:mod.rs-0237 */ ///     // That closure is fine, but now we can't use `ok` anywhere else,
/* FP:mod.rs-0238 */ ///     // since it is owned by the previous task:
/* FP:mod.rs-0239 */ ///     // s.spawn(|_| println!("ok: {:?}", ok));
/* FP:mod.rs-0240 */ /// });
/* FP:mod.rs-0241 */ /// ```
/* FP:mod.rs-0242 */ ///
/* FP:mod.rs-0243 */ /// While this works, it could be a problem if we want to use `ok` elsewhere.
/* FP:mod.rs-0244 */ /// There are two choices. We can keep the closure as a `move` closure, but
/* FP:mod.rs-0245 */ /// instead of referencing the variable `ok`, we create a shadowed variable that
/* FP:mod.rs-0246 */ /// is a borrow of `ok` and capture *that*:
/* FP:mod.rs-0247 */ ///
/* FP:mod.rs-0248 */ /// ```rust
/* FP:mod.rs-0249 */ /// # use rustc_thread_pool as rayon;
/* FP:mod.rs-0250 */ /// let ok: Vec<i32> = vec![1, 2, 3];
/* FP:mod.rs-0251 */ /// rayon::scope(|s| {
/* FP:mod.rs-0252 */ ///     let bad: Vec<i32> = vec![4, 5, 6];
/* FP:mod.rs-0253 */ ///     let ok: &Vec<i32> = &ok; // shadow the original `ok`
/* FP:mod.rs-0254 */ ///     s.spawn(move |_| {
/* FP:mod.rs-0255 */ ///         println!("ok: {:?}", ok); // captures the shadowed version
/* FP:mod.rs-0256 */ ///         println!("bad: {:?}", bad);
/* FP:mod.rs-0257 */ ///     });
/* FP:mod.rs-0258 */ ///
/* FP:mod.rs-0259 */ ///     // Now we too can use the shadowed `ok`, since `&Vec<i32>` references
/* FP:mod.rs-0260 */ ///     // can be shared freely. Note that we need a `move` closure here though,
/* FP:mod.rs-0261 */ ///     // because otherwise we'd be trying to borrow the shadowed `ok`,
/* FP:mod.rs-0262 */ ///     // and that doesn't outlive `scope`.
/* FP:mod.rs-0263 */ ///     s.spawn(move |_| println!("ok: {:?}", ok));
/* FP:mod.rs-0264 */ /// });
/* FP:mod.rs-0265 */ /// ```
/* FP:mod.rs-0266 */ ///
/* FP:mod.rs-0267 */ /// Another option is not to use the `move` keyword but instead to take ownership
/* FP:mod.rs-0268 */ /// of individual variables:
/* FP:mod.rs-0269 */ ///
/* FP:mod.rs-0270 */ /// ```rust
/* FP:mod.rs-0271 */ /// # use rustc_thread_pool as rayon;
/* FP:mod.rs-0272 */ /// let ok: Vec<i32> = vec![1, 2, 3];
/* FP:mod.rs-0273 */ /// rayon::scope(|s| {
/* FP:mod.rs-0274 */ ///     let bad: Vec<i32> = vec![4, 5, 6];
/* FP:mod.rs-0275 */ ///     s.spawn(|_| {
/* FP:mod.rs-0276 */ ///         // Transfer ownership of `bad` into a local variable (also named `bad`).
/* FP:mod.rs-0277 */ ///         // This will force the closure to take ownership of `bad` from the environment.
/* FP:mod.rs-0278 */ ///         let bad = bad;
/* FP:mod.rs-0279 */ ///         println!("ok: {:?}", ok); // `ok` is only borrowed.
/* FP:mod.rs-0280 */ ///         println!("bad: {:?}", bad); // refers to our local variable, above.
/* FP:mod.rs-0281 */ ///     });
/* FP:mod.rs-0282 */ ///
/* FP:mod.rs-0283 */ ///     s.spawn(|_| println!("ok: {:?}", ok)); // we too can borrow `ok`
/* FP:mod.rs-0284 */ /// });
/* FP:mod.rs-0285 */ /// ```
/* FP:mod.rs-0286 */ ///
/* FP:mod.rs-0287 */ /// # Panics
/* FP:mod.rs-0288 */ ///
/* FP:mod.rs-0289 */ /// If a panic occurs, either in the closure given to `scope()` or in
/* FP:mod.rs-0290 */ /// any of the spawned jobs, that panic will be propagated and the
/* FP:mod.rs-0291 */ /// call to `scope()` will panic. If multiple panics occurs, it is
/* FP:mod.rs-0292 */ /// non-deterministic which of their panic values will propagate.
/* FP:mod.rs-0293 */ /// Regardless, once a task is spawned using `scope.spawn()`, it will
/* FP:mod.rs-0294 */ /// execute, even if the spawning task should later panic. `scope()`
/* FP:mod.rs-0295 */ /// returns once all spawned jobs have completed, and any panics are
/* FP:mod.rs-0296 */ /// propagated at that point.
/* FP:mod.rs-0297 */ pub fn scope<'scope, OP, R>(op: OP) -> R
/* FP:mod.rs-0298 */ where
/* FP:mod.rs-0299 */     OP: FnOnce(&Scope<'scope>) -> R + Send,
/* FP:mod.rs-0300 */     R: Send,
/* FP:mod.rs-0301 */ {
/* FP:mod.rs-0302 */     in_worker(|owner_thread, _| {
/* FP:mod.rs-0303 */         let scope = Scope::<'scope>::new(Some(owner_thread), None);
/* FP:mod.rs-0304 */         scope.base.complete(Some(owner_thread), || op(&scope))
/* FP:mod.rs-0305 */     })
/* FP:mod.rs-0306 */ }
/* FP:mod.rs-0307 */ 
/* FP:mod.rs-0308 */ /// Creates a "fork-join" scope `s` with FIFO order, and invokes the
/* FP:mod.rs-0309 */ /// closure with a reference to `s`. This closure can then spawn
/* FP:mod.rs-0310 */ /// asynchronous tasks into `s`. Those tasks may run asynchronously with
/* FP:mod.rs-0311 */ /// respect to the closure; they may themselves spawn additional tasks
/* FP:mod.rs-0312 */ /// into `s`. When the closure returns, it will block until all tasks
/* FP:mod.rs-0313 */ /// that have been spawned into `s` complete.
/* FP:mod.rs-0314 */ ///
/* FP:mod.rs-0315 */ /// # Task execution
/* FP:mod.rs-0316 */ ///
/* FP:mod.rs-0317 */ /// Tasks in a `scope_fifo()` run similarly to [`scope()`], but there's a
/* FP:mod.rs-0318 */ /// difference in the order of execution. Consider a similar example:
/* FP:mod.rs-0319 */ ///
/* FP:mod.rs-0320 */ /// [`scope()`]: fn.scope.html
/* FP:mod.rs-0321 */ ///
/* FP:mod.rs-0322 */ /// ```rust
/* FP:mod.rs-0323 */ /// # use rustc_thread_pool as rayon;
/* FP:mod.rs-0324 */ /// // point start
/* FP:mod.rs-0325 */ /// rayon::scope_fifo(|s| {
/* FP:mod.rs-0326 */ ///     s.spawn_fifo(|s| { // task s.1
/* FP:mod.rs-0327 */ ///         s.spawn_fifo(|s| { // task s.1.1
/* FP:mod.rs-0328 */ ///             rayon::scope_fifo(|t| {
/* FP:mod.rs-0329 */ ///                 t.spawn_fifo(|_| ()); // task t.1
/* FP:mod.rs-0330 */ ///                 t.spawn_fifo(|_| ()); // task t.2
/* FP:mod.rs-0331 */ ///             });
/* FP:mod.rs-0332 */ ///         });
/* FP:mod.rs-0333 */ ///     });
/* FP:mod.rs-0334 */ ///     s.spawn_fifo(|s| { // task s.2
/* FP:mod.rs-0335 */ ///     });
/* FP:mod.rs-0336 */ ///     // point mid
/* FP:mod.rs-0337 */ /// });
/* FP:mod.rs-0338 */ /// // point end
/* FP:mod.rs-0339 */ /// ```
/* FP:mod.rs-0340 */ ///
/* FP:mod.rs-0341 */ /// The various tasks that are run will execute roughly like so:
/* FP:mod.rs-0342 */ ///
/* FP:mod.rs-0343 */ /// ```notrust
/* FP:mod.rs-0344 */ /// | (start)
/* FP:mod.rs-0345 */ /// |
/* FP:mod.rs-0346 */ /// | (FIFO scope `s` created)
/* FP:mod.rs-0347 */ /// +--------------------+ (task s.1)
/* FP:mod.rs-0348 */ /// +-------+ (task s.2) |
/* FP:mod.rs-0349 */ /// |       |            +---+ (task s.1.1)
/* FP:mod.rs-0350 */ /// |       |            |   |
/* FP:mod.rs-0351 */ /// |       |            |   | (FIFO scope `t` created)
/* FP:mod.rs-0352 */ /// |       |            |   +----------------+ (task t.1)
/* FP:mod.rs-0353 */ /// |       |            |   +---+ (task t.2) |
/* FP:mod.rs-0354 */ /// | (mid) |            |   |   |            |
/* FP:mod.rs-0355 */ /// :       |            |   + <-+------------+ (scope `t` ends)
/* FP:mod.rs-0356 */ /// :       |            |   |
/* FP:mod.rs-0357 */ /// |<------+------------+---+ (scope `s` ends)
/* FP:mod.rs-0358 */ /// |
/* FP:mod.rs-0359 */ /// | (end)
/* FP:mod.rs-0360 */ /// ```
/* FP:mod.rs-0361 */ ///
/* FP:mod.rs-0362 */ /// Under `scope_fifo()`, the spawns are prioritized in a FIFO order on
/* FP:mod.rs-0363 */ /// the thread from which they were spawned, as opposed to `scope()`'s
/* FP:mod.rs-0364 */ /// LIFO. So in this example, we can expect `s.1` to execute before
/* FP:mod.rs-0365 */ /// `s.2`, and `t.1` before `t.2`. Other threads also steal tasks in
/* FP:mod.rs-0366 */ /// FIFO order, as usual. Overall, this has roughly the same order as
/* FP:mod.rs-0367 */ /// the now-deprecated [`breadth_first`] option, except the effect is
/* FP:mod.rs-0368 */ /// isolated to a particular scope. If spawns are intermingled from any
/* FP:mod.rs-0369 */ /// combination of `scope()` and `scope_fifo()`, or from different
/* FP:mod.rs-0370 */ /// threads, their order is only specified with respect to spawns in the
/* FP:mod.rs-0371 */ /// same scope and thread.
/* FP:mod.rs-0372 */ ///
/* FP:mod.rs-0373 */ /// For more details on this design, see Rayon [RFC #1].
/* FP:mod.rs-0374 */ ///
/* FP:mod.rs-0375 */ /// [`breadth_first`]: struct.ThreadPoolBuilder.html#method.breadth_first
/* FP:mod.rs-0376 */ /// [RFC #1]: https://github.com/rayon-rs/rfcs/blob/master/accepted/rfc0001-scope-scheduling.md
/* FP:mod.rs-0377 */ ///
/* FP:mod.rs-0378 */ /// # Panics
/* FP:mod.rs-0379 */ ///
/* FP:mod.rs-0380 */ /// If a panic occurs, either in the closure given to `scope_fifo()` or
/* FP:mod.rs-0381 */ /// in any of the spawned jobs, that panic will be propagated and the
/* FP:mod.rs-0382 */ /// call to `scope_fifo()` will panic. If multiple panics occurs, it is
/* FP:mod.rs-0383 */ /// non-deterministic which of their panic values will propagate.
/* FP:mod.rs-0384 */ /// Regardless, once a task is spawned using `scope.spawn_fifo()`, it
/* FP:mod.rs-0385 */ /// will execute, even if the spawning task should later panic.
/* FP:mod.rs-0386 */ /// `scope_fifo()` returns once all spawned jobs have completed, and any
/* FP:mod.rs-0387 */ /// panics are propagated at that point.
/* FP:mod.rs-0388 */ pub fn scope_fifo<'scope, OP, R>(op: OP) -> R
/* FP:mod.rs-0389 */ where
/* FP:mod.rs-0390 */     OP: FnOnce(&ScopeFifo<'scope>) -> R + Send,
/* FP:mod.rs-0391 */     R: Send,
/* FP:mod.rs-0392 */ {
/* FP:mod.rs-0393 */     in_worker(|owner_thread, _| {
/* FP:mod.rs-0394 */         let scope = ScopeFifo::<'scope>::new(Some(owner_thread), None);
/* FP:mod.rs-0395 */         scope.base.complete(Some(owner_thread), || op(&scope))
/* FP:mod.rs-0396 */     })
/* FP:mod.rs-0397 */ }
/* FP:mod.rs-0398 */ 
/* FP:mod.rs-0399 */ /// Creates a "fork-join" scope `s` and invokes the closure with a
/* FP:mod.rs-0400 */ /// reference to `s`. This closure can then spawn asynchronous tasks
/* FP:mod.rs-0401 */ /// into `s`. Those tasks may run asynchronously with respect to the
/* FP:mod.rs-0402 */ /// closure; they may themselves spawn additional tasks into `s`. When
/* FP:mod.rs-0403 */ /// the closure returns, it will block until all tasks that have been
/* FP:mod.rs-0404 */ /// spawned into `s` complete.
/* FP:mod.rs-0405 */ ///
/* FP:mod.rs-0406 */ /// This is just like `scope()` except the closure runs on the same thread
/* FP:mod.rs-0407 */ /// that calls `in_place_scope()`. Only work that it spawns runs in the
/* FP:mod.rs-0408 */ /// thread pool.
/* FP:mod.rs-0409 */ ///
/* FP:mod.rs-0410 */ /// # Panics
/* FP:mod.rs-0411 */ ///
/* FP:mod.rs-0412 */ /// If a panic occurs, either in the closure given to `in_place_scope()` or in
/* FP:mod.rs-0413 */ /// any of the spawned jobs, that panic will be propagated and the
/* FP:mod.rs-0414 */ /// call to `in_place_scope()` will panic. If multiple panics occurs, it is
/* FP:mod.rs-0415 */ /// non-deterministic which of their panic values will propagate.
/* FP:mod.rs-0416 */ /// Regardless, once a task is spawned using `scope.spawn()`, it will
/* FP:mod.rs-0417 */ /// execute, even if the spawning task should later panic. `in_place_scope()`
/* FP:mod.rs-0418 */ /// returns once all spawned jobs have completed, and any panics are
/* FP:mod.rs-0419 */ /// propagated at that point.
/* FP:mod.rs-0420 */ pub fn in_place_scope<'scope, OP, R>(op: OP) -> R
/* FP:mod.rs-0421 */ where
/* FP:mod.rs-0422 */     OP: FnOnce(&Scope<'scope>) -> R,
/* FP:mod.rs-0423 */ {
/* FP:mod.rs-0424 */     do_in_place_scope(None, op)
/* FP:mod.rs-0425 */ }
/* FP:mod.rs-0426 */ 
/* FP:mod.rs-0427 */ pub(crate) fn do_in_place_scope<'scope, OP, R>(registry: Option<&Arc<Registry>>, op: OP) -> R
/* FP:mod.rs-0428 */ where
/* FP:mod.rs-0429 */     OP: FnOnce(&Scope<'scope>) -> R,
/* FP:mod.rs-0430 */ {
/* FP:mod.rs-0431 */     let thread = unsafe { WorkerThread::current().as_ref() };
/* FP:mod.rs-0432 */     let scope = Scope::<'scope>::new(thread, registry);
/* FP:mod.rs-0433 */     scope.base.complete(thread, || op(&scope))
/* FP:mod.rs-0434 */ }
/* FP:mod.rs-0435 */ 
/* FP:mod.rs-0436 */ /// Creates a "fork-join" scope `s` with FIFO order, and invokes the
/* FP:mod.rs-0437 */ /// closure with a reference to `s`. This closure can then spawn
/* FP:mod.rs-0438 */ /// asynchronous tasks into `s`. Those tasks may run asynchronously with
/* FP:mod.rs-0439 */ /// respect to the closure; they may themselves spawn additional tasks
/* FP:mod.rs-0440 */ /// into `s`. When the closure returns, it will block until all tasks
/* FP:mod.rs-0441 */ /// that have been spawned into `s` complete.
/* FP:mod.rs-0442 */ ///
/* FP:mod.rs-0443 */ /// This is just like `scope_fifo()` except the closure runs on the same thread
/* FP:mod.rs-0444 */ /// that calls `in_place_scope_fifo()`. Only work that it spawns runs in the
/* FP:mod.rs-0445 */ /// thread pool.
/* FP:mod.rs-0446 */ ///
/* FP:mod.rs-0447 */ /// # Panics
/* FP:mod.rs-0448 */ ///
/* FP:mod.rs-0449 */ /// If a panic occurs, either in the closure given to `in_place_scope_fifo()` or in
/* FP:mod.rs-0450 */ /// any of the spawned jobs, that panic will be propagated and the
/* FP:mod.rs-0451 */ /// call to `in_place_scope_fifo()` will panic. If multiple panics occurs, it is
/* FP:mod.rs-0452 */ /// non-deterministic which of their panic values will propagate.
/* FP:mod.rs-0453 */ /// Regardless, once a task is spawned using `scope.spawn_fifo()`, it will
/* FP:mod.rs-0454 */ /// execute, even if the spawning task should later panic. `in_place_scope_fifo()`
/* FP:mod.rs-0455 */ /// returns once all spawned jobs have completed, and any panics are
/* FP:mod.rs-0456 */ /// propagated at that point.
/* FP:mod.rs-0457 */ pub fn in_place_scope_fifo<'scope, OP, R>(op: OP) -> R
/* FP:mod.rs-0458 */ where
/* FP:mod.rs-0459 */     OP: FnOnce(&ScopeFifo<'scope>) -> R,
/* FP:mod.rs-0460 */ {
/* FP:mod.rs-0461 */     do_in_place_scope_fifo(None, op)
/* FP:mod.rs-0462 */ }
/* FP:mod.rs-0463 */ 
/* FP:mod.rs-0464 */ pub(crate) fn do_in_place_scope_fifo<'scope, OP, R>(registry: Option<&Arc<Registry>>, op: OP) -> R
/* FP:mod.rs-0465 */ where
/* FP:mod.rs-0466 */     OP: FnOnce(&ScopeFifo<'scope>) -> R,
/* FP:mod.rs-0467 */ {
/* FP:mod.rs-0468 */     let thread = unsafe { WorkerThread::current().as_ref() };
/* FP:mod.rs-0469 */     let scope = ScopeFifo::<'scope>::new(thread, registry);
/* FP:mod.rs-0470 */     scope.base.complete(thread, || op(&scope))
/* FP:mod.rs-0471 */ }
/* FP:mod.rs-0472 */ 
/* FP:mod.rs-0473 */ impl<'scope> Scope<'scope> {
/* FP:mod.rs-0474 */     fn new(owner: Option<&WorkerThread>, registry: Option<&Arc<Registry>>) -> Self {
/* FP:mod.rs-0475 */         let base = ScopeBase::new(owner, registry);
/* FP:mod.rs-0476 */         Scope { base }
/* FP:mod.rs-0477 */     }
/* FP:mod.rs-0478 */ 
/* FP:mod.rs-0479 */     /// Spawns a job into the fork-join scope `self`. This job will
/* FP:mod.rs-0480 */     /// execute sometime before the fork-join scope completes. The
/* FP:mod.rs-0481 */     /// job is specified as a closure, and this closure receives its
/* FP:mod.rs-0482 */     /// own reference to the scope `self` as argument. This can be
/* FP:mod.rs-0483 */     /// used to inject new jobs into `self`.
/* FP:mod.rs-0484 */     ///
/* FP:mod.rs-0485 */     /// # Returns
/* FP:mod.rs-0486 */     ///
/* FP:mod.rs-0487 */     /// Nothing. The spawned closures cannot pass back values to the
/* FP:mod.rs-0488 */     /// caller directly, though they can write to local variables on
/* FP:mod.rs-0489 */     /// the stack (if those variables outlive the scope) or
/* FP:mod.rs-0490 */     /// communicate through shared channels.
/* FP:mod.rs-0491 */     ///
/* FP:mod.rs-0492 */     /// (The intention is to eventually integrate with Rust futures to
/* FP:mod.rs-0493 */     /// support spawns of functions that compute a value.)
/* FP:mod.rs-0494 */     ///
/* FP:mod.rs-0495 */     /// # Examples
/* FP:mod.rs-0496 */     ///
/* FP:mod.rs-0497 */     /// ```rust
/* FP:mod.rs-0498 */     /// # use rustc_thread_pool as rayon;
/* FP:mod.rs-0499 */     /// let mut value_a = None;
/* FP:mod.rs-0500 */     /// let mut value_b = None;
/* FP:mod.rs-0501 */     /// let mut value_c = None;
/* FP:mod.rs-0502 */     /// rayon::scope(|s| {
/* FP:mod.rs-0503 */     ///     s.spawn(|s1| {
/* FP:mod.rs-0504 */     ///         //   ^ this is the same scope as `s`; this handle `s1`
/* FP:mod.rs-0505 */     ///         //     is intended for use by the spawned task,
/* FP:mod.rs-0506 */     ///         //     since scope handles cannot cross thread boundaries.
/* FP:mod.rs-0507 */     ///
/* FP:mod.rs-0508 */     ///         value_a = Some(22);
/* FP:mod.rs-0509 */     ///
/* FP:mod.rs-0510 */     ///         // the scope `s` will not end until all these tasks are done
/* FP:mod.rs-0511 */     ///         s1.spawn(|_| {
/* FP:mod.rs-0512 */     ///             value_b = Some(44);
/* FP:mod.rs-0513 */     ///         });
/* FP:mod.rs-0514 */     ///     });
/* FP:mod.rs-0515 */     ///
/* FP:mod.rs-0516 */     ///     s.spawn(|_| {
/* FP:mod.rs-0517 */     ///         value_c = Some(66);
/* FP:mod.rs-0518 */     ///     });
/* FP:mod.rs-0519 */     /// });
/* FP:mod.rs-0520 */     /// assert_eq!(value_a, Some(22));
/* FP:mod.rs-0521 */     /// assert_eq!(value_b, Some(44));
/* FP:mod.rs-0522 */     /// assert_eq!(value_c, Some(66));
/* FP:mod.rs-0523 */     /// ```
/* FP:mod.rs-0524 */     ///
/* FP:mod.rs-0525 */     /// # See also
/* FP:mod.rs-0526 */     ///
/* FP:mod.rs-0527 */     /// The [`scope` function] has more extensive documentation about
/* FP:mod.rs-0528 */     /// task spawning.
/* FP:mod.rs-0529 */     ///
/* FP:mod.rs-0530 */     /// [`scope` function]: fn.scope.html
/* FP:mod.rs-0531 */     pub fn spawn<BODY>(&self, body: BODY)
/* FP:mod.rs-0532 */     where
/* FP:mod.rs-0533 */         BODY: FnOnce(&Scope<'scope>) + Send + 'scope,
/* FP:mod.rs-0534 */     {
/* FP:mod.rs-0535 */         let scope_ptr = ScopePtr(self);
/* FP:mod.rs-0536 */         let job = HeapJob::new(self.base.tlv, move |id| unsafe {
/* FP:mod.rs-0537 */             // SAFETY: this job will execute before the scope ends.
/* FP:mod.rs-0538 */             let scope = scope_ptr.as_ref();
/* FP:mod.rs-0539 */ 
/* FP:mod.rs-0540 */             // Mark this job is started.
/* FP:mod.rs-0541 */             scope.base.pending_jobs.lock().unwrap().remove(&id);
/* FP:mod.rs-0542 */ 
/* FP:mod.rs-0543 */             ScopeBase::execute_job(&scope.base, move || body(scope))
/* FP:mod.rs-0544 */         });
/* FP:mod.rs-0545 */         let job_ref = self.base.heap_job_ref(job);
/* FP:mod.rs-0546 */ 
/* FP:mod.rs-0547 */         // Mark this job as pending.
/* FP:mod.rs-0548 */         self.base.pending_jobs.lock().unwrap().insert(job_ref.id());
/* FP:mod.rs-0549 */         // Since `Scope` implements `Sync`, we can't be sure that we're still in a
/* FP:mod.rs-0550 */         // thread of this pool, so we can't just push to the local worker thread.
/* FP:mod.rs-0551 */         // Also, this might be an in-place scope.
/* FP:mod.rs-0552 */         self.base.registry.inject_or_push(job_ref);
/* FP:mod.rs-0553 */     }
/* FP:mod.rs-0554 */ 
/* FP:mod.rs-0555 */     /// Spawns a job into every thread of the fork-join scope `self`. This job will
/* FP:mod.rs-0556 */     /// execute on each thread sometime before the fork-join scope completes. The
/* FP:mod.rs-0557 */     /// job is specified as a closure, and this closure receives its own reference
/* FP:mod.rs-0558 */     /// to the scope `self` as argument, as well as a `BroadcastContext`.
/* FP:mod.rs-0559 */     pub fn spawn_broadcast<BODY>(&self, body: BODY)
/* FP:mod.rs-0560 */     where
/* FP:mod.rs-0561 */         BODY: Fn(&Scope<'scope>, BroadcastContext<'_>) + Send + Sync + 'scope,
/* FP:mod.rs-0562 */     {
/* FP:mod.rs-0563 */         let scope_ptr = ScopePtr(self);
/* FP:mod.rs-0564 */         let job = ArcJob::new(move |id| unsafe {
/* FP:mod.rs-0565 */             // SAFETY: this job will execute before the scope ends.
/* FP:mod.rs-0566 */             let scope = scope_ptr.as_ref();
/* FP:mod.rs-0567 */             let body = &body;
/* FP:mod.rs-0568 */ 
/* FP:mod.rs-0569 */             let current_index = WorkerThread::current().as_ref().map(|worker| worker.index());
/* FP:mod.rs-0570 */             if current_index == scope.base.worker {
/* FP:mod.rs-0571 */                 // Mark this job as started on the scope's worker thread.
/* FP:mod.rs-0572 */                 scope.base.pending_jobs.lock().unwrap().remove(&id);
/* FP:mod.rs-0573 */             }
/* FP:mod.rs-0574 */ 
/* FP:mod.rs-0575 */             let func = move || BroadcastContext::with(move |ctx| body(scope, ctx));
/* FP:mod.rs-0576 */             ScopeBase::execute_job(&scope.base, func)
/* FP:mod.rs-0577 */         });
/* FP:mod.rs-0578 */         self.base.inject_broadcast(job)
/* FP:mod.rs-0579 */     }
/* FP:mod.rs-0580 */ }
/* FP:mod.rs-0581 */ 
/* FP:mod.rs-0582 */ impl<'scope> ScopeFifo<'scope> {
/* FP:mod.rs-0583 */     fn new(owner: Option<&WorkerThread>, registry: Option<&Arc<Registry>>) -> Self {
/* FP:mod.rs-0584 */         let base = ScopeBase::new(owner, registry);
/* FP:mod.rs-0585 */         let num_threads = base.registry.num_threads();
/* FP:mod.rs-0586 */         let fifos = (0..num_threads).map(|_| JobFifo::new()).collect();
/* FP:mod.rs-0587 */         ScopeFifo { base, fifos }
/* FP:mod.rs-0588 */     }
/* FP:mod.rs-0589 */ 
/* FP:mod.rs-0590 */     /// Spawns a job into the fork-join scope `self`. This job will
/* FP:mod.rs-0591 */     /// execute sometime before the fork-join scope completes. The
/* FP:mod.rs-0592 */     /// job is specified as a closure, and this closure receives its
/* FP:mod.rs-0593 */     /// own reference to the scope `self` as argument. This can be
/* FP:mod.rs-0594 */     /// used to inject new jobs into `self`.
/* FP:mod.rs-0595 */     ///
/* FP:mod.rs-0596 */     /// # See also
/* FP:mod.rs-0597 */     ///
/* FP:mod.rs-0598 */     /// This method is akin to [`Scope::spawn()`], but with a FIFO
/* FP:mod.rs-0599 */     /// priority. The [`scope_fifo` function] has more details about
/* FP:mod.rs-0600 */     /// this distinction.
/* FP:mod.rs-0601 */     ///
/* FP:mod.rs-0602 */     /// [`Scope::spawn()`]: struct.Scope.html#method.spawn
/* FP:mod.rs-0603 */     /// [`scope_fifo` function]: fn.scope_fifo.html
/* FP:mod.rs-0604 */     pub fn spawn_fifo<BODY>(&self, body: BODY)
/* FP:mod.rs-0605 */     where
/* FP:mod.rs-0606 */         BODY: FnOnce(&ScopeFifo<'scope>) + Send + 'scope,
/* FP:mod.rs-0607 */     {
/* FP:mod.rs-0608 */         let scope_ptr = ScopePtr(self);
/* FP:mod.rs-0609 */         let job = HeapJob::new(self.base.tlv, move |id| unsafe {
/* FP:mod.rs-0610 */             // SAFETY: this job will execute before the scope ends.
/* FP:mod.rs-0611 */             let scope = scope_ptr.as_ref();
/* FP:mod.rs-0612 */ 
/* FP:mod.rs-0613 */             // Mark this job is started.
/* FP:mod.rs-0614 */             scope.base.pending_jobs.lock().unwrap().remove(&id);
/* FP:mod.rs-0615 */ 
/* FP:mod.rs-0616 */             ScopeBase::execute_job(&scope.base, move || body(scope))
/* FP:mod.rs-0617 */         });
/* FP:mod.rs-0618 */         let job_ref = self.base.heap_job_ref(job);
/* FP:mod.rs-0619 */ 
/* FP:mod.rs-0620 */         // Mark this job as pending.
/* FP:mod.rs-0621 */         self.base.pending_jobs.lock().unwrap().insert(job_ref.id());
/* FP:mod.rs-0622 */ 
/* FP:mod.rs-0623 */         // Since `ScopeFifo` implements `Sync`, we can't be sure that we're still in a
/* FP:mod.rs-0624 */         // thread of this pool, so we can't just push to the local worker thread.
/* FP:mod.rs-0625 */         // Also, this might be an in-place scope.
/* FP:mod.rs-0626 */         self.base.registry.inject_or_push(job_ref);
/* FP:mod.rs-0627 */     }
/* FP:mod.rs-0628 */ 
/* FP:mod.rs-0629 */     /// Spawns a job into every thread of the fork-join scope `self`. This job will
/* FP:mod.rs-0630 */     /// execute on each thread sometime before the fork-join scope completes. The
/* FP:mod.rs-0631 */     /// job is specified as a closure, and this closure receives its own reference
/* FP:mod.rs-0632 */     /// to the scope `self` as argument, as well as a `BroadcastContext`.
/* FP:mod.rs-0633 */     pub fn spawn_broadcast<BODY>(&self, body: BODY)
/* FP:mod.rs-0634 */     where
/* FP:mod.rs-0635 */         BODY: Fn(&ScopeFifo<'scope>, BroadcastContext<'_>) + Send + Sync + 'scope,
/* FP:mod.rs-0636 */     {
/* FP:mod.rs-0637 */         let scope_ptr = ScopePtr(self);
/* FP:mod.rs-0638 */         let job = ArcJob::new(move |id| unsafe {
/* FP:mod.rs-0639 */             // SAFETY: this job will execute before the scope ends.
/* FP:mod.rs-0640 */             let scope = scope_ptr.as_ref();
/* FP:mod.rs-0641 */ 
/* FP:mod.rs-0642 */             let current_index = WorkerThread::current().as_ref().map(|worker| worker.index());
/* FP:mod.rs-0643 */             if current_index == scope.base.worker {
/* FP:mod.rs-0644 */                 // Mark this job as started on the scope's worker thread.
/* FP:mod.rs-0645 */                 scope.base.pending_jobs.lock().unwrap().remove(&id);
/* FP:mod.rs-0646 */             }
/* FP:mod.rs-0647 */             let body = &body;
/* FP:mod.rs-0648 */             let func = move || BroadcastContext::with(move |ctx| body(scope, ctx));
/* FP:mod.rs-0649 */             ScopeBase::execute_job(&scope.base, func)
/* FP:mod.rs-0650 */         });
/* FP:mod.rs-0651 */         self.base.inject_broadcast(job)
/* FP:mod.rs-0652 */     }
/* FP:mod.rs-0653 */ }
/* FP:mod.rs-0654 */ 
/* FP:mod.rs-0655 */ impl<'scope> ScopeBase<'scope> {
/* FP:mod.rs-0656 */     /// Creates the base of a new scope for the given registry
/* FP:mod.rs-0657 */     fn new(owner: Option<&WorkerThread>, registry: Option<&Arc<Registry>>) -> Self {
/* FP:mod.rs-0658 */         let registry = registry.unwrap_or_else(|| match owner {
/* FP:mod.rs-0659 */             Some(owner) => owner.registry(),
/* FP:mod.rs-0660 */             None => global_registry(),
/* FP:mod.rs-0661 */         });
/* FP:mod.rs-0662 */ 
/* FP:mod.rs-0663 */         ScopeBase {
/* FP:mod.rs-0664 */             registry: Arc::clone(registry),
/* FP:mod.rs-0665 */             panic: AtomicPtr::new(ptr::null_mut()),
/* FP:mod.rs-0666 */             job_completed_latch: CountLatch::new(owner),
/* FP:mod.rs-0667 */             #[allow(rustc::default_hash_types)]
/* FP:mod.rs-0668 */             pending_jobs: Mutex::new(HashSet::new()),
/* FP:mod.rs-0669 */             worker: owner.map(|w| w.index()),
/* FP:mod.rs-0670 */             marker: PhantomData,
/* FP:mod.rs-0671 */             tlv: tlv::get(),
/* FP:mod.rs-0672 */         }
/* FP:mod.rs-0673 */     }
/* FP:mod.rs-0674 */ 
/* FP:mod.rs-0675 */     fn heap_job_ref<FUNC>(&self, job: Box<HeapJob<FUNC>>) -> JobRef
/* FP:mod.rs-0676 */     where
/* FP:mod.rs-0677 */         FUNC: FnOnce(JobRefId) + Send + 'scope,
/* FP:mod.rs-0678 */     {
/* FP:mod.rs-0679 */         unsafe {
/* FP:mod.rs-0680 */             self.job_completed_latch.increment();
/* FP:mod.rs-0681 */             job.into_job_ref()
/* FP:mod.rs-0682 */         }
/* FP:mod.rs-0683 */     }
/* FP:mod.rs-0684 */ 
/* FP:mod.rs-0685 */     fn inject_broadcast<FUNC>(&self, job: Arc<ArcJob<FUNC>>)
/* FP:mod.rs-0686 */     where
/* FP:mod.rs-0687 */         FUNC: Fn(JobRefId) + Send + Sync + 'scope,
/* FP:mod.rs-0688 */     {
/* FP:mod.rs-0689 */         if self.worker.is_some() {
/* FP:mod.rs-0690 */             let id = unsafe { ArcJob::as_job_ref(&job).id() };
/* FP:mod.rs-0691 */             self.pending_jobs.lock().unwrap().insert(id);
/* FP:mod.rs-0692 */         }
/* FP:mod.rs-0693 */         let n_threads = self.registry.num_threads();
/* FP:mod.rs-0694 */         let job_refs = (0..n_threads).map(|_| unsafe {
/* FP:mod.rs-0695 */             self.job_completed_latch.increment();
/* FP:mod.rs-0696 */             ArcJob::as_job_ref(&job)
/* FP:mod.rs-0697 */         });
/* FP:mod.rs-0698 */ 
/* FP:mod.rs-0699 */         self.registry.inject_broadcast(job_refs);
/* FP:mod.rs-0700 */     }
/* FP:mod.rs-0701 */ 
/* FP:mod.rs-0702 */     /// Executes `func` as a job, either aborting or executing as
/* FP:mod.rs-0703 */     /// appropriate.
/* FP:mod.rs-0704 */     fn complete<FUNC, R>(&self, owner: Option<&WorkerThread>, func: FUNC) -> R
/* FP:mod.rs-0705 */     where
/* FP:mod.rs-0706 */         FUNC: FnOnce() -> R,
/* FP:mod.rs-0707 */     {
/* FP:mod.rs-0708 */         let result = unsafe { Self::execute_job_closure(self, func) };
/* FP:mod.rs-0709 */         self.job_completed_latch.wait(
/* FP:mod.rs-0710 */             owner,
/* FP:mod.rs-0711 */             || self.pending_jobs.lock().unwrap().is_empty(),
/* FP:mod.rs-0712 */             |job| self.pending_jobs.lock().unwrap().contains(&job.id()),
/* FP:mod.rs-0713 */         );
/* FP:mod.rs-0714 */ 
/* FP:mod.rs-0715 */         // Restore the TLV if we ran some jobs while waiting
/* FP:mod.rs-0716 */         tlv::set(self.tlv);
/* FP:mod.rs-0717 */ 
/* FP:mod.rs-0718 */         self.maybe_propagate_panic();
/* FP:mod.rs-0719 */         result.unwrap() // only None if `op` panicked, and that would have been propagated
/* FP:mod.rs-0720 */     }
/* FP:mod.rs-0721 */ 
/* FP:mod.rs-0722 */     /// Executes `func` as a job, either aborting or executing as
/* FP:mod.rs-0723 */     /// appropriate.
/* FP:mod.rs-0724 */     unsafe fn execute_job<FUNC>(this: *const Self, func: FUNC)
/* FP:mod.rs-0725 */     where
/* FP:mod.rs-0726 */         FUNC: FnOnce(),
/* FP:mod.rs-0727 */     {
/* FP:mod.rs-0728 */         let _: Option<()> = unsafe { Self::execute_job_closure(this, func) };
/* FP:mod.rs-0729 */     }
/* FP:mod.rs-0730 */ 
/* FP:mod.rs-0731 */     /// Executes `func` as a job in scope. Adjusts the "job completed"
/* FP:mod.rs-0732 */     /// counters and also catches any panic and stores it into
/* FP:mod.rs-0733 */     /// `scope`.
/* FP:mod.rs-0734 */     unsafe fn execute_job_closure<FUNC, R>(this: *const Self, func: FUNC) -> Option<R>
/* FP:mod.rs-0735 */     where
/* FP:mod.rs-0736 */         FUNC: FnOnce() -> R,
/* FP:mod.rs-0737 */     {
/* FP:mod.rs-0738 */         let result = match unwind::halt_unwinding(func) {
/* FP:mod.rs-0739 */             Ok(r) => Some(r),
/* FP:mod.rs-0740 */             Err(err) => {
/* FP:mod.rs-0741 */                 unsafe { (*this).job_panicked(err) };
/* FP:mod.rs-0742 */                 None
/* FP:mod.rs-0743 */             }
/* FP:mod.rs-0744 */         };
/* FP:mod.rs-0745 */         unsafe { Latch::set(&(*this).job_completed_latch) };
/* FP:mod.rs-0746 */         result
/* FP:mod.rs-0747 */     }
/* FP:mod.rs-0748 */ 
/* FP:mod.rs-0749 */     fn job_panicked(&self, err: Box<dyn Any + Send + 'static>) {
/* FP:mod.rs-0750 */         // capture the first error we see, free the rest
/* FP:mod.rs-0751 */         if self.panic.load(Ordering::Relaxed).is_null() {
/* FP:mod.rs-0752 */             let nil = ptr::null_mut();
/* FP:mod.rs-0753 */             let mut err = ManuallyDrop::new(Box::new(err)); // box up the fat ptr
/* FP:mod.rs-0754 */             let err_ptr: *mut Box<dyn Any + Send + 'static> = &mut **err;
/* FP:mod.rs-0755 */             if self
/* FP:mod.rs-0756 */                 .panic
/* FP:mod.rs-0757 */                 .compare_exchange(nil, err_ptr, Ordering::Release, Ordering::Relaxed)
/* FP:mod.rs-0758 */                 .is_ok()
/* FP:mod.rs-0759 */             {
/* FP:mod.rs-0760 */                 // ownership now transferred into self.panic
/* FP:mod.rs-0761 */             } else {
/* FP:mod.rs-0762 */                 // another panic raced in ahead of us, so drop ours
/* FP:mod.rs-0763 */                 let _: Box<Box<_>> = ManuallyDrop::into_inner(err);
/* FP:mod.rs-0764 */             }
/* FP:mod.rs-0765 */         }
/* FP:mod.rs-0766 */     }
/* FP:mod.rs-0767 */ 
/* FP:mod.rs-0768 */     fn maybe_propagate_panic(&self) {
/* FP:mod.rs-0769 */         // propagate panic, if any occurred; at this point, all
/* FP:mod.rs-0770 */         // outstanding jobs have completed, so we can use a relaxed
/* FP:mod.rs-0771 */         // ordering:
/* FP:mod.rs-0772 */         let panic = self.panic.swap(ptr::null_mut(), Ordering::Relaxed);
/* FP:mod.rs-0773 */         if !panic.is_null() {
/* FP:mod.rs-0774 */             let value = unsafe { Box::from_raw(panic) };
/* FP:mod.rs-0775 */ 
/* FP:mod.rs-0776 */             // Restore the TLV if we ran some jobs while waiting
/* FP:mod.rs-0777 */             tlv::set(self.tlv);
/* FP:mod.rs-0778 */ 
/* FP:mod.rs-0779 */             unwind::resume_unwinding(*value);
/* FP:mod.rs-0780 */         }
/* FP:mod.rs-0781 */     }
/* FP:mod.rs-0782 */ }
/* FP:mod.rs-0783 */ 
/* FP:mod.rs-0784 */ impl<'scope> fmt::Debug for Scope<'scope> {
/* FP:mod.rs-0785 */     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:mod.rs-0786 */         fmt.debug_struct("Scope")
/* FP:mod.rs-0787 */             .field("pool_id", &self.base.registry.id())
/* FP:mod.rs-0788 */             .field("panic", &self.base.panic)
/* FP:mod.rs-0789 */             .field("job_completed_latch", &self.base.job_completed_latch)
/* FP:mod.rs-0790 */             .finish()
/* FP:mod.rs-0791 */     }
/* FP:mod.rs-0792 */ }
/* FP:mod.rs-0793 */ 
/* FP:mod.rs-0794 */ impl<'scope> fmt::Debug for ScopeFifo<'scope> {
/* FP:mod.rs-0795 */     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:mod.rs-0796 */         fmt.debug_struct("ScopeFifo")
/* FP:mod.rs-0797 */             .field("num_fifos", &self.fifos.len())
/* FP:mod.rs-0798 */             .field("pool_id", &self.base.registry.id())
/* FP:mod.rs-0799 */             .field("panic", &self.base.panic)
/* FP:mod.rs-0800 */             .field("job_completed_latch", &self.base.job_completed_latch)
/* FP:mod.rs-0801 */             .finish()
/* FP:mod.rs-0802 */     }
/* FP:mod.rs-0803 */ }
/* FP:mod.rs-0804 */ 
/* FP:mod.rs-0805 */ /// Used to capture a scope `&Self` pointer in jobs, without faking a lifetime.
/* FP:mod.rs-0806 */ ///
/* FP:mod.rs-0807 */ /// Unsafe code is still required to dereference the pointer, but that's fine in
/* FP:mod.rs-0808 */ /// scope jobs that are guaranteed to execute before the scope ends.
/* FP:mod.rs-0809 */ struct ScopePtr<T>(*const T);
/* FP:mod.rs-0810 */ 
/* FP:mod.rs-0811 */ // SAFETY: !Send for raw pointers is not for safety, just as a lint
/* FP:mod.rs-0812 */ unsafe impl<T: Sync> Send for ScopePtr<T> {}
/* FP:mod.rs-0813 */ 
/* FP:mod.rs-0814 */ // SAFETY: !Sync for raw pointers is not for safety, just as a lint
/* FP:mod.rs-0815 */ unsafe impl<T: Sync> Sync for ScopePtr<T> {}
/* FP:mod.rs-0816 */ 
/* FP:mod.rs-0817 */ impl<T> ScopePtr<T> {
/* FP:mod.rs-0818 */     // Helper to avoid disjoint captures of `scope_ptr.0`
/* FP:mod.rs-0819 */     unsafe fn as_ref(&self) -> &T {
/* FP:mod.rs-0820 */         unsafe { &*self.0 }
/* FP:mod.rs-0821 */     }
/* FP:mod.rs-0822 */ }