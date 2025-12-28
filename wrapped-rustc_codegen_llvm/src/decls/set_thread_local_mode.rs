macro_rules! deps {
    () => {
        ThreadLocalMode!();
    };
}

macro_rules! set_thread_local_mode {
    () => {
        deps!();
        pub (crate) fn set_thread_local_mode (global : & Value , mode : ThreadLocalMode) { unsafe { LLVMSetThreadLocalMode (global , mode) ; } }
    };
}

set_thread_local_mode!();