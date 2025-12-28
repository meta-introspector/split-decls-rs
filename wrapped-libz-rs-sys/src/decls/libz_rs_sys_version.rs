macro_rules! libz_rs_sys_version {
    () => {
        macro_rules ! libz_rs_sys_version { () => { concat ! ("1.3.0-zlib-rs-" , env ! ("CARGO_PKG_VERSION") , "\0") } ; }
    };
}

libz_rs_sys_version!()