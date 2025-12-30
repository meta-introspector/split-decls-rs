// Generated macro for impl_605 (impl)
macro_rules! Depcrate_ossl_paramimpl_605 {
() => {
// Module: crate::ossl_param
// Provides: {"impl_605"}
// Dependencies: {}
impl OsslParamArray { # [doc = " Locates the individual `OSSL_PARAM` element representing an"] # [doc = " octet string identified by the key in the `OsslParamArray`"] # [doc = " array and returns a reference to it."] # [doc = ""] # [doc = " Combines OSSL_PARAM_locate and OSSL_PARAM_get_octet_string."] # [corresponds (OSSL_PARAM_get_octet_string)] # [allow (dead_code)] pub (crate) fn locate_octet_string < 'a > (& 'a self , key : & CStr) -> Result < & 'a [u8] , ErrorStack > { unsafe { let param = cvt_p (ffi :: OSSL_PARAM_locate (self . as_ptr () , key . as_ptr ())) ? ; let mut val : * const c_void = ptr :: null_mut () ; let mut val_len : usize = 0 ; cvt (ffi :: OSSL_PARAM_get_octet_string_ptr (param , & mut val , & mut val_len ,)) ? ; Ok (util :: from_raw_parts (val as * const u8 , val_len)) } } }
};
}
