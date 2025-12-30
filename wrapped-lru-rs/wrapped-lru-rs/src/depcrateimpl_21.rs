// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl < K : ? Sized > KeyWrapper < K > { fn from_ref (key : & K) -> & Self { unsafe { & * (key as * const K as * const KeyWrapper < K >) } } }
};
}
