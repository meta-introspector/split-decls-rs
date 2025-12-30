// Generated macro for should_write_ir (function)
macro_rules! Depcrate_pretty_clifshould_write_ir {
() => {
// Module: crate::pretty_clif
// Provides: {"should_write_ir"}
// Dependencies: {}
pub (crate) fn should_write_ir (tcx : TyCtxt < '_ >) -> bool { tcx . sess . opts . output_types . contains_key (& OutputType :: LlvmAssembly) }
};
}
