macro_rules! DEFAULT_BUG_REPORT_URL {
    () => {
        pub const DEFAULT_BUG_REPORT_URL : & str = "https://github.com/rust-lang/rust/issues/new\
    ?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md" ;
    };
}

DEFAULT_BUG_REPORT_URL!()