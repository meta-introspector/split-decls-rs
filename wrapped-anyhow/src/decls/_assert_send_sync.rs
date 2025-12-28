macro_rules! deps {
    () => {
        Backtrace!();
    };
}

macro_rules! _assert_send_sync {
    () => {
        deps!();
        fn _assert_send_sync () { fn assert < T : Send + Sync > () { } assert :: < Backtrace > () ; }
    };
}

_assert_send_sync!()