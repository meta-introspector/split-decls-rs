// Generated macro for impl_31 (impl)
macro_rules! Depcrate_attrimpl_31 {
() => {
// Module: crate::attr
// Provides: {"impl_31"}
// Dependencies: {}
impl SvalAttribute for UnlabeledVariantsAttr { type Result = bool ; fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Bool (ref b) = lit { b . value } else { panic ! ("unexpected value") } } }
};
}
