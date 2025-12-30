// Generated macro for slice_transform_name_callback (function)
macro_rules! Depcrate_slice_transformslice_transform_name_callback {
() => {
// Module: crate::slice_transform
// Provides: {"slice_transform_name_callback"}
// Dependencies: {}
pub unsafe extern "C" fn slice_transform_name_callback (raw_cb : * mut c_void) -> * const c_char { let cb = unsafe { & mut * (raw_cb as * mut TransformCallback) } ; cb . name . as_ptr () }
};
}
