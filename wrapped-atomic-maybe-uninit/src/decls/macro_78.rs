macro_rules! macro_78 {
    () => {
        cfg_has_atomic_32 ! { test_atomic ! (i32) ; test_atomic ! (u32) ; stress_test ! (u32) ; }
    };
}

macro_78!()