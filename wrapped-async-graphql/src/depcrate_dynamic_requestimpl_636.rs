// Generated macro for impl_636 (impl)
macro_rules! Depcrate_dynamic_requestimpl_636 {
() => {
// Module: crate::dynamic::request
// Provides: {"impl_636"}
// Dependencies: {}
impl < T : Into < Request > > From < T > for DynamicRequest { fn from (req : T) -> Self { Self { inner : req . into () , root_value : FieldValue :: NULL , } } }
};
}
