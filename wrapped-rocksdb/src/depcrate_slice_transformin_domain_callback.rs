// Generated macro for in_domain_callback (function)
macro_rules! Depcrate_slice_transformin_domain_callback {
() => {
// Module: crate::slice_transform
// Provides: {"in_domain_callback"}
// Dependencies: {}
pub unsafe extern "C" fn in_domain_callback (raw_cb : * mut c_void , raw_key : * const c_char , key_len : size_t ,) -> c_uchar { let cb = unsafe { & mut * (raw_cb as * mut TransformCallback) } ; let key = unsafe { slice :: from_raw_parts (raw_key as * const u8 , key_len) } ; c_uchar :: from (cb . in_domain_fn . is_none_or (| in_domain | in_domain (key))) }
};
}
