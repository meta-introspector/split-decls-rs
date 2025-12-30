// Generated macro for type_for_type_alias_with_diagnostics_cycle_result (function)
macro_rules! Depcrate_lowertype_for_type_alias_with_diagnostics_cycle_result {
() => {
// Module: crate::lower
// Provides: {"type_for_type_alias_with_diagnostics_cycle_result"}
// Dependencies: {}
pub (crate) fn type_for_type_alias_with_diagnostics_cycle_result < 'db > (db : & 'db dyn HirDatabase , _adt : TypeAliasId ,) -> (EarlyBinder < 'db , Ty < 'db > > , Diagnostics) { (EarlyBinder :: bind (Ty :: new_error (DbInterner :: new_with (db , None , None) , ErrorGuaranteed)) , None) }
};
}
