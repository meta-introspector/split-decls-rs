macro_rules! deps {
    () => {
        SYNCHRONIZATION_ACCESS_RIGHTS!();
    };
}

macro_rules! SEMAPHORE_MODIFY_STATE {
    () => {
        deps!();
        pub const SEMAPHORE_MODIFY_STATE : SYNCHRONIZATION_ACCESS_RIGHTS = 2u32 ;
    };
}

SEMAPHORE_MODIFY_STATE!()