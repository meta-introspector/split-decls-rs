// Generated macro for impl_1124 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1124 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1124"}
// Dependencies: {}
# [stable (feature = "cstring_from_vec_of_nonzerou8" , since = "1.43.0")] impl From < Vec < NonZero < u8 > > > for CString { # [doc = " Converts a <code>[Vec]<[NonZero]<[u8]>></code> into a [`CString`] without"] # [doc = " copying nor checking for inner nul bytes."] # [inline] fn from (v : Vec < NonZero < u8 > >) -> CString { unsafe { let v : Vec < u8 > = { let (ptr , len , cap) : (* mut NonZero < u8 > , _ , _) = Vec :: into_raw_parts (v) ; Vec :: from_raw_parts (ptr . cast :: < u8 > () , len , cap) } ; Self :: _from_vec_unchecked (v) } } }
};
}
