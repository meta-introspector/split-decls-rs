macro_rules! deps {
    () => {
        THREAD_ACCESS_RIGHTS!();
    };
}

macro_rules! THREAD_SYNCHRONIZE {
    () => {
        deps!();
        pub const THREAD_SYNCHRONIZE : THREAD_ACCESS_RIGHTS = 1048576u32 ;
    };
}

THREAD_SYNCHRONIZE!()