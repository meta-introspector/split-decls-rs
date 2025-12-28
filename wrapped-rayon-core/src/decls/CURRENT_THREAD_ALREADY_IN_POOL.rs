macro_rules! CURRENT_THREAD_ALREADY_IN_POOL {
    () => {
        const CURRENT_THREAD_ALREADY_IN_POOL : & str = "The current thread is already part of another thread pool." ;
    };
}

CURRENT_THREAD_ALREADY_IN_POOL!();