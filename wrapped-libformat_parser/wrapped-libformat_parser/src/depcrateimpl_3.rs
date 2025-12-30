// Generated macro for impl_3 (impl)
macro_rules! Depcrateimpl_3 {
() => {
// Module: crate
// Provides: {"impl_3"}
// Dependencies: {}
impl < T > IntoFFI < * const T > for Option < T > where T : Sized , { fn into_ffi (self) -> * const T { match self . as_ref () { None => std :: ptr :: null () , Some (r) => r as * const T , } } }
};
}
