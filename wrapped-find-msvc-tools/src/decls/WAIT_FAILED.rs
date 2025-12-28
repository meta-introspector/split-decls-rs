macro_rules! deps {
    () => {
        WAIT_EVENT!();
    };
}

macro_rules! WAIT_FAILED {
    () => {
        deps!();
        pub const WAIT_FAILED : WAIT_EVENT = 4294967295u32 ;
    };
}

WAIT_FAILED!();