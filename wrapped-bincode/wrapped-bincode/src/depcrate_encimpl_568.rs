// Generated macro for impl_568 (impl)
macro_rules! Depcrate_encimpl_568 {
() => {
// Module: crate::enc
// Provides: {"impl_568"}
// Dependencies: {}
impl < T > Encoder for & mut T where T : Encoder , { type W = T :: W ; type C = T :: C ; fn writer (& mut self) -> & mut Self :: W { T :: writer (self) } fn config (& self) -> & Self :: C { T :: config (self) } }
};
}
