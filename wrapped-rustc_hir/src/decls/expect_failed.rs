macro_rules! expect_failed {
    () => {
        # [track_caller] fn expect_failed < T : fmt :: Debug > (ident : & 'static str , found : T) -> ! { panic ! ("{ident}: found {found:?}") }
    };
}

expect_failed!();