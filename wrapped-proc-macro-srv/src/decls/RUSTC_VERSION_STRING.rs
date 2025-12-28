macro_rules! RUSTC_VERSION_STRING {
    () => {
        pub const RUSTC_VERSION_STRING : & str = env ! ("RUSTC_VERSION") ;
    };
}

RUSTC_VERSION_STRING!();