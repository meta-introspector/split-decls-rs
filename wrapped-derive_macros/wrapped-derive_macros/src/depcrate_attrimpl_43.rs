// Generated macro for impl_43 (impl)
macro_rules! Depcrate_attrimpl_43 {
() => {
// Module: crate::attr
// Provides: {"impl_43"}
// Dependencies: {}
impl SvalAttribute for FlattenAttr { type Result = bool ; fn from_lit (& self , lit : & Lit) -> Self :: Result { # [cfg (not (feature = "flatten"))] { let _ = lit ; panic ! ("the `flatten` attribute can only be used when the `flatten` Cargo feature of `sval_derive` is enabled") ; } # [cfg (feature = "flatten")] { if let Lit :: Bool (ref b) = lit { b . value } else { panic ! ("unexpected value") } } } }
};
}
