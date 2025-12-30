// Generated macro for FromVoid (trait)
macro_rules! Depcrate_baseFromVoid {
() => {
// Module: crate::base
// Provides: {"FromVoid"}
// Dependencies: {}
# [doc = " A trait describing how to convert from the stored `*const c_void` to the desired `T`"] pub unsafe trait FromVoid { unsafe fn from_void < 'a > (x : * const c_void) -> ItemRef < 'a , Self > where Self : std :: marker :: Sized ; }
};
}
