macro_rules! deps {
    () => {
        LocalNode!();
    };
}

macro_rules! THREAD_HEAD {
    () => {
        deps!();
        # [cfg (feature = "experimental-thread-local")] # [thread_local] # [doc = " A debt node assigned to this thread."] static THREAD_HEAD : OnceCell < LocalNode > = OnceCell :: new () ;
    };
}

THREAD_HEAD!();