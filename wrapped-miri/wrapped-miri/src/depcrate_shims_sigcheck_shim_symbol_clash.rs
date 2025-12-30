// Generated macro for check_shim_symbol_clash (function)
macro_rules! Depcrate_shims_sigcheck_shim_symbol_clash {
() => {
// Module: crate::shims::sig
// Provides: {"check_shim_symbol_clash"}
// Dependencies: {}
fn check_shim_symbol_clash < 'tcx > (this : & mut MiriInterpCx < 'tcx > , link_name : Symbol ,) -> InterpResult < 'tcx , () > { if let Some ((body , instance)) = this . lookup_exported_symbol (link_name) ? { if this . tcx . is_compiler_builtins (instance . def_id () . krate) { return interp_ok (()) ; } throw_machine_stop ! (TerminationInfo :: SymbolShimClashing { link_name , span : body . span . data () , }) } interp_ok (()) }
};
}
