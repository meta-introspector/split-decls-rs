// Generated macro for Stage (trait)
macro_rules! Depcrate_contextStage {
() => {
// Module: crate::context
// Provides: {"Stage"}
// Dependencies: {}
# [allow (private_interfaces)] pub trait Stage : Sized + 'static + Sealed { type Id : Copy ; fn parsers () -> & 'static GroupType < Self > ; fn emit_err < 'sess > (& self , sess : & 'sess Session , diag : impl for < 'x > Diagnostic < 'x > ,) -> ErrorGuaranteed ; fn should_emit (& self) -> ShouldEmit ; fn id_is_crate_root (id : Self :: Id) -> bool ; }
};
}
