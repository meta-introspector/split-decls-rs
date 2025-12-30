// Generated macro for impl_635 (impl)
macro_rules! Depcrate_dynamic_requestimpl_635 {
() => {
// Module: crate::dynamic::request
// Provides: {"impl_635"}
// Dependencies: {}
impl < T : Into < Request > > DynamicRequestExt for T { fn root_value (self , value : FieldValue < 'static >) -> DynamicRequest { DynamicRequest { inner : self . into () , root_value : value , } } }
};
}
