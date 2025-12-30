// Generated macro for impl_28 (impl)
macro_rules! Depcrate_attrimpl_28 {
() => {
// Module: crate::attr
// Provides: {"impl_28"}
// Dependencies: {}
impl SvalAttribute for UnindexedFieldsAttr { type Result = bool ; fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Bool (ref b) = lit { b . value } else { panic ! ("unexpected value") } } }
};
}
