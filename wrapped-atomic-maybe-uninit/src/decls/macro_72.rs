macro_rules! macro_72 {
    () => {
        cfg_has_atomic_8 ! { test_atomic ! (i8) ; test_atomic ! (u8) ; stress_test ! (u8) ; }
    };
}

macro_72!();