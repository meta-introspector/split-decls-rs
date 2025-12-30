// Generated macro for Direction (trait)
macro_rules! Depcrate_ffi_cDirection {
() => {
// Module: crate::ffi::c
// Provides: {"Direction"}
// Dependencies: {}
# [doc = " Trait used to call the right destroy/end function on the inner"] # [doc = " stream object on drop."] pub trait Direction { unsafe fn destroy (stream : * mut mz_stream) -> c_int ; }
};
}
