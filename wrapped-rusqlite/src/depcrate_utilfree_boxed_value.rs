// Generated macro for free_boxed_value (function)
macro_rules! Depcrate_utilfree_boxed_value {
() => {
// Module: crate::util
// Provides: {"free_boxed_value"}
// Dependencies: {}
# [cfg (any (feature = "collation" , feature = "functions" , feature = "vtab"))] pub (crate) unsafe extern "C" fn free_boxed_value < T > (p : * mut std :: ffi :: c_void) { drop (Box :: from_raw (p . cast :: < T > ())) ; }
};
}
