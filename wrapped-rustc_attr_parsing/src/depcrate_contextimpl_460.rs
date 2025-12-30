// Generated macro for impl_460 (impl)
macro_rules! Depcrate_contextimpl_460 {
() => {
// Module: crate::context
// Provides: {"impl_460"}
// Dependencies: {}
# [allow (private_interfaces)] impl Stage for Early { type Id = NodeId ; fn parsers () -> & 'static GroupType < Self > { & early :: ATTRIBUTE_PARSERS } fn emit_err < 'sess > (& self , sess : & 'sess Session , diag : impl for < 'x > Diagnostic < 'x > ,) -> ErrorGuaranteed { self . should_emit () . emit_err (sess . dcx () . create_err (diag)) } fn should_emit (& self) -> ShouldEmit { self . emit_errors } fn id_is_crate_root (id : Self :: Id) -> bool { id == CRATE_NODE_ID } }
};
}
