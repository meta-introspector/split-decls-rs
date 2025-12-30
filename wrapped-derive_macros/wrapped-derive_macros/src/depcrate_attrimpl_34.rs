// Generated macro for impl_34 (impl)
macro_rules! Depcrate_attrimpl_34 {
() => {
// Module: crate::attr
// Provides: {"impl_34"}
// Dependencies: {}
impl SvalAttribute for UnindexedVariantsAttr { type Result = bool ; fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Bool (ref b) = lit { b . value } else { panic ! ("unexpected value") } } }
};
}
