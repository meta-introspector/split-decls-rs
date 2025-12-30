// Generated macro for impl_37 (impl)
macro_rules! Depcrate_attrimpl_37 {
() => {
// Module: crate::attr
// Provides: {"impl_37"}
// Dependencies: {}
impl SvalAttribute for DynamicAttr { type Result = bool ; fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Bool (ref b) = lit { b . value } else { panic ! ("unexpected value") } } }
};
}
