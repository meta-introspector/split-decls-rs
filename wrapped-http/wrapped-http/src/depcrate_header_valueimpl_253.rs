// Generated macro for impl_253 (impl)
macro_rules! Depcrate_header_valueimpl_253 {
() => {
// Module: crate::header::value
// Provides: {"impl_253"}
// Dependencies: {}
impl < 'a , T : ? Sized > PartialOrd < & 'a T > for HeaderValue where HeaderValue : PartialOrd < T > , { # [inline] fn partial_cmp (& self , other : & & 'a T) -> Option < cmp :: Ordering > { self . partial_cmp (* other) } }
};
}
