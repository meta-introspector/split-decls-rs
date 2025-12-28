macro_rules! deps {
    () => {
        Scope!();
        ScopedJoinHandle!();
        ScopedThreadBuilder!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < 'env > Scope < 'env > { # [doc = " Spawns a scoped thread."] # [doc = ""] # [doc = " This method is similar to the [`spawn`] function in Rust's standard library. The difference"] # [doc = " is that this thread is scoped, meaning it's guaranteed to terminate before the scope exits,"] # [doc = " allowing it to reference variables outside the scope."] # [doc = ""] # [doc = " The scoped thread is passed a reference to this scope as an argument, which can be used for"] # [doc = " spawning nested threads."] # [doc = ""] # [doc = " The returned [handle](ScopedJoinHandle) can be used to manually"] # [doc = " [join](ScopedJoinHandle::join) the thread before the scope exits."] # [doc = ""] # [doc = " This will create a thread using default parameters of [`ScopedThreadBuilder`], if you want to specify the"] # [doc = " stack size or the name of the thread, use this API instead."] # [doc = ""] # [doc = " [`spawn`]: std::thread::spawn"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the OS fails to create a thread; use [`ScopedThreadBuilder::spawn`]"] # [doc = " to recover from such errors."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::thread;"] # [doc = ""] # [doc = " thread::scope(|s| {"] # [doc = "     let handle = s.spawn(|_| {"] # [doc = "         println!(\"A child thread is running\");"] # [doc = "         42"] # [doc = "     });"] # [doc = ""] # [doc = "     // Join the thread and retrieve its result."] # [doc = "     let res = handle.join().unwrap();"] # [doc = "     assert_eq!(res, 42);"] # [doc = " }).unwrap();"] # [doc = " ```"] pub fn spawn < 'scope , F , T > (& 'scope self , f : F) -> ScopedJoinHandle < 'scope , T > where F : FnOnce (& Scope < 'env >) -> T , F : Send + 'env , T : Send + 'env , { self . builder () . spawn (f) . expect ("failed to spawn scoped thread") } # [doc = " Creates a builder that can configure a thread before spawning."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::thread;"] # [doc = ""] # [doc = " thread::scope(|s| {"] # [doc = "     s.builder()"] # [doc = "         .spawn(|_| println!(\"A child thread is running\"))"] # [doc = "         .unwrap();"] # [doc = " }).unwrap();"] # [doc = " ```"] pub fn builder < 'scope > (& 'scope self) -> ScopedThreadBuilder < 'scope , 'env > { ScopedThreadBuilder { scope : self , builder : thread :: Builder :: new () , } } }
    };
}

impl_157!();