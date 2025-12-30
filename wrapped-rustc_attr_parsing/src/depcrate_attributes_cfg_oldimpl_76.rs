// Generated macro for impl_76 (impl)
macro_rules! Depcrate_attributes_cfg_oldimpl_76 {
() => {
// Module: crate::attributes::cfg_old
// Provides: {"impl_76"}
// Dependencies: {}
impl CfgMatchesLintEmitter for NodeId { fn emit_span_lint (& self , sess : & Session , lint : & 'static Lint , sp : Span , diag : BuiltinLintDiag) { sess . psess . buffer_lint (lint , sp , * self , diag) ; } }
};
}
