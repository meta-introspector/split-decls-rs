// Generated macro for impl_40 (impl)
macro_rules! Depcrate_attrimpl_40 {
() => {
// Module: crate::attr
// Provides: {"impl_40"}
// Dependencies: {}
impl SvalAttribute for TransparentAttr { type Result = bool ; fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Bool (ref b) = lit { b . value } else { panic ! ("unexpected value") } } }
};
}
