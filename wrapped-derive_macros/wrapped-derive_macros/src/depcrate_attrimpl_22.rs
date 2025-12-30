// Generated macro for impl_22 (impl)
macro_rules! Depcrate_attrimpl_22 {
() => {
// Module: crate::attr
// Provides: {"impl_22"}
// Dependencies: {}
impl SvalAttribute for SkipAttr { type Result = bool ; fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Bool (ref b) = lit { b . value } else { panic ! ("unexpected value") } } }
};
}
