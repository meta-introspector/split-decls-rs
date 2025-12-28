macro_rules! deps {
    () => {
        JoinHandle!();
    };
}

macro_rules! _assert_traits {
    () => {
        deps!();
        fn _assert_traits () { fn assert < T : Send + Sync > () { } assert :: < JoinHandle < () > > () ; }
    };
}

_assert_traits!();