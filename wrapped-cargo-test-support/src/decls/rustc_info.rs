macro_rules! deps {
    () => {
        RustcInfo!();
    };
}

macro_rules! rustc_info {
    () => {
        deps!();
        fn rustc_info () -> & 'static RustcInfo { static RUSTC_INFO : OnceLock < RustcInfo > = OnceLock :: new () ; RUSTC_INFO . get_or_init (RustcInfo :: new) }
    };
}

rustc_info!()