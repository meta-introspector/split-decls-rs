// Generated macro for FromMutVoid (trait)
macro_rules! Depcrate_baseFromMutVoid {
() => {
// Module: crate::base
// Provides: {"FromMutVoid"}
// Dependencies: {}
# [doc = " A trait describing how to convert from the stored `*mut c_void` to the desired `T`"] pub unsafe trait FromMutVoid { unsafe fn from_mut_void < 'a > (x : * mut c_void) -> ItemMutRef < 'a , Self > where Self : std :: marker :: Sized ; }
};
}
