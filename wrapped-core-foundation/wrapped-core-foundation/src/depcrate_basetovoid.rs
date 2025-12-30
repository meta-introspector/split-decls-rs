// Generated macro for ToVoid (trait)
macro_rules! Depcrate_baseToVoid {
() => {
// Module: crate::base
// Provides: {"ToVoid"}
// Dependencies: {}
# [doc = " A trait describing how to convert from the stored `*const c_void` to the desired `T`"] pub unsafe trait ToVoid < T > { fn to_void (& self) -> * const c_void ; }
};
}
