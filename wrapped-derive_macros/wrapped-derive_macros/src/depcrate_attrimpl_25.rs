// Generated macro for impl_25 (impl)
macro_rules! Depcrate_attrimpl_25 {
() => {
// Module: crate::attr
// Provides: {"impl_25"}
// Dependencies: {}
impl SvalAttribute for UnlabeledFieldsAttr { type Result = bool ; fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Bool (ref b) = lit { b . value } else { panic ! ("unexpected value") } } }
};
}
