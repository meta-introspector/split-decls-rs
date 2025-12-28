macro_rules! VERSION_CSTR {
    () => {
        # [doc = " cbindgen:ignore"] static VERSION_CSTR : & str = concat ! (env ! ("CARGO_PKG_VERSION") , "\0") ;
    };
}

VERSION_CSTR!()