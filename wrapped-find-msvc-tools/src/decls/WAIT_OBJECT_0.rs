macro_rules! deps {
    () => {
        WAIT_EVENT!();
    };
}

macro_rules! WAIT_OBJECT_0 {
    () => {
        deps!();
        pub const WAIT_OBJECT_0 : WAIT_EVENT = 0u32 ;
    };
}

WAIT_OBJECT_0!();