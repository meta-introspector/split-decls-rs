macro_rules! macro_69 {
    () => {
        cfg_has_atomic_ptr ! { test_atomic ! (isize) ; test_atomic ! (usize) ; }
    };
}

macro_69!()