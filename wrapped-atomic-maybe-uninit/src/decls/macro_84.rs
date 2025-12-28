macro_rules! macro_84 {
    () => {
        # [cfg (not (all (valgrind , target_arch = "powerpc64")))] cfg_has_atomic_128 ! { test_atomic ! (i128) ; test_atomic ! (u128) ; stress_test ! (u128) ; }
    };
}

macro_84!();