macro_rules! deps {
    () => {
        DlInfo!();
    };
}

macro_rules! other_103 {
    () => {
        deps!();
        # [cfg_attr (any (target_os = "linux" , target_os = "android") , link (name = "dl"))] # [cfg_attr (any (target_os = "freebsd" , target_os = "dragonfly") , link (name = "c"))] extern "C" { fn dlopen (filename : * const core :: ffi :: c_char , flags : core :: ffi :: c_int ,) -> * mut core :: ffi :: c_void ; fn dlclose (handle : * mut core :: ffi :: c_void) -> core :: ffi :: c_int ; fn dlsym (handle : * mut core :: ffi :: c_void , symbol : * const core :: ffi :: c_char ,) -> * mut core :: ffi :: c_void ; fn dlerror () -> * mut core :: ffi :: c_char ; fn dladdr (addr : * mut core :: ffi :: c_void , info : * mut DlInfo) -> core :: ffi :: c_int ; }
    };
}

other_103!()