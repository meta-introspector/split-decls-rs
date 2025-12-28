macro_rules! deps {
    () => {
        WAIT_EVENT!();
    };
}

macro_rules! WAIT_ABANDONED {
    () => {
        deps!();
        pub const WAIT_ABANDONED : WAIT_EVENT = 128u32 ;
    };
}

WAIT_ABANDONED!();