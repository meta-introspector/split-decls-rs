// Generated macro for impl_1268 (impl)
macro_rules! Depcrateimpl_1268 {
() => {
// Module: crate
// Provides: {"impl_1268"}
// Dependencies: {}
impl CodegenLintLevels { pub fn from_tcx (tcx : TyCtxt < '_ >) -> Self { Self { linker_messages : tcx . lint_level_at_node (LINKER_MESSAGES , CRATE_HIR_ID) } } }
};
}
