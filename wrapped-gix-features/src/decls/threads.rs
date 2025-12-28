macro_rules! deps {
    () => {
        Scope!();
    };
}

macro_rules! threads {
    () => {
        deps!();
        # [doc = " Runs `f` with a scope to be used for spawning threads that will not outlive the function call."] # [doc = " That way it's possible to handle threads without needing the 'static lifetime for data they interact with."] # [doc = ""] # [doc = " Note that the threads should not rely on actual parallelism as threading might be turned off entirely, hence should not"] # [doc = " connect each other with channels as deadlock would occur in single-threaded mode."] pub fn threads < 'env , F , R > (f : F) -> R where F : for < 'scope > FnOnce (& 'scope std :: thread :: Scope < 'scope , 'env >) -> R , { std :: thread :: scope (f) }
    };
}

threads!()