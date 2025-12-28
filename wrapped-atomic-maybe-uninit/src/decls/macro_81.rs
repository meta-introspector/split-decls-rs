macro_rules! macro_81 {
    () => {
        cfg_has_atomic_64 ! { test_atomic ! (i64) ; test_atomic ! (u64) ; stress_test ! (u64) ; }
    };
}

macro_81!();