macro_rules! RustcInfo {
    () => {
        struct RustcInfo { verbose_version : String , host : String , }
    };
}

RustcInfo!();