macro_rules! deps {
    () => {
        Incoming!();
    };
}

macro_rules! _assert_send_sync {
    () => {
        deps!();
        fn _assert_send_sync () { fn _assert_send < T : Send > () { } fn _assert_sync < T : Sync > () { } _assert_send :: < Incoming > () ; _assert_sync :: < Incoming > () ; }
    };
}

_assert_send_sync!();