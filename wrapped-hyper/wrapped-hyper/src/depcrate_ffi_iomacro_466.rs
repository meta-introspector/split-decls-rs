// Generated macro for macro_466 (macro)
macro_rules! Depcrate_ffi_iomacro_466 {
() => {
// Module: crate::ffi::io
// Provides: {"macro_466"}
// Dependencies: {}
ffi_fn ! { # [doc = " Set the user data pointer for this IO to some value."] # [doc = ""] # [doc = " This value is passed as an argument to the read and write callbacks."] fn hyper_io_set_userdata (io : * mut hyper_io , data : * mut c_void) { non_null ! (& mut * io ?= ()) . userdata = data ; } }
};
}
