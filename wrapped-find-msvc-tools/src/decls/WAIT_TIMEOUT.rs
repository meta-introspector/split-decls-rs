macro_rules! deps {
    () => {
        WAIT_EVENT!();
    };
}

macro_rules! WAIT_TIMEOUT {
    () => {
        deps!();
        pub const WAIT_TIMEOUT : WAIT_EVENT = 258u32 ;
    };
}

WAIT_TIMEOUT!();