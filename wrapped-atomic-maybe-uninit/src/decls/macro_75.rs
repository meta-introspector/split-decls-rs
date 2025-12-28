macro_rules! macro_75 {
    () => {
        cfg_has_atomic_16 ! { test_atomic ! (i16) ; test_atomic ! (u16) ; stress_test ! (u16) ; }
    };
}

macro_75!();