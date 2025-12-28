macro_rules! deps {
    () => {
        Profiler!();
    };
}

macro_rules! _assert_bounds {
    () => {
        deps!();
        fn _assert_bounds () { assert_bounds_inner (& Profiler :: new ("")) ; fn assert_bounds_inner < S : Sized + Send + Sync + 'static > (_ : & S) { } }
    };
}

_assert_bounds!();