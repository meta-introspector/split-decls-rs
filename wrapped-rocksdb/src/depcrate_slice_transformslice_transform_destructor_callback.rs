// Generated macro for slice_transform_destructor_callback (function)
macro_rules! Depcrate_slice_transformslice_transform_destructor_callback {
() => {
// Module: crate::slice_transform
// Provides: {"slice_transform_destructor_callback"}
// Dependencies: {}
pub unsafe extern "C" fn slice_transform_destructor_callback (raw_cb : * mut c_void) { drop (unsafe { Box :: from_raw (raw_cb as * mut TransformCallback) }) ; }
};
}
