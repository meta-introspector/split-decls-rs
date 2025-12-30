// Generated macro for reject_outside_std (macro)
macro_rules! Depcrate_attributes_stabilityreject_outside_std {
() => {
// Module: crate::attributes::stability
// Provides: {"reject_outside_std"}
// Dependencies: {}
macro_rules ! reject_outside_std { ($ cx : ident) => { if !$ cx . features () . staged_api () { $ cx . emit_err (session_diagnostics :: StabilityOutsideStd { span : $ cx . attr_span }) ; return ; } } ; }
};
}
