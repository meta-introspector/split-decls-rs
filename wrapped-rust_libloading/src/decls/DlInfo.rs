macro_rules! DlInfo {
    () => {
        # [repr (C)] struct DlInfo { dli_fname : * const core :: ffi :: c_char , dli_fbase : * mut core :: ffi :: c_void , dli_sname : * const core :: ffi :: c_char , dli_saddr : * mut core :: ffi :: c_void , }
    };
}

DlInfo!()