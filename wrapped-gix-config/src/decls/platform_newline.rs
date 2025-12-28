macro_rules! platform_newline {
    () => {
        pub (crate) fn platform_newline () -> & 'static BStr { if cfg ! (windows) { "\r\n" } else { "\n" } . into () }
    };
}

platform_newline!()