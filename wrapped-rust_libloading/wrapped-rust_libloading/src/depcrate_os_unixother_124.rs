// Generated macro for other_124 (other)
macro_rules! Depcrate_os_unixother_124 {
() => {
// Module: crate::os::unix
// Provides: {"other_124"}
// Dependencies: {}
# [cfg_attr (any (target_os = "linux" , target_os = "android") , link (name = "dl"))] # [cfg_attr (any (target_os = "freebsd" , target_os = "dragonfly") , link (name = "c"))] extern "C" { fn dlopen (filename : * const core :: ffi :: c_char , flags : core :: ffi :: c_int ,) -> * mut core :: ffi :: c_void ; fn dlclose (handle : * mut core :: ffi :: c_void) -> core :: ffi :: c_int ; fn dlsym (handle : * mut core :: ffi :: c_void , symbol : * const core :: ffi :: c_char ,) -> * mut core :: ffi :: c_void ; fn dlerror () -> * mut core :: ffi :: c_char ; fn dladdr (addr : * mut core :: ffi :: c_void , info : * mut DlInfo) -> core :: ffi :: c_int ; }
};
}
