macro_rules! deps {
    () => {
        COINIT!();
    };
}

macro_rules! COINIT_MULTITHREADED {
    () => {
        deps!();
        pub const COINIT_MULTITHREADED : COINIT = 0i32 ;
    };
}

COINIT_MULTITHREADED!()