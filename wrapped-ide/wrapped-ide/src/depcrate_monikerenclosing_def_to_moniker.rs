// Generated macro for enclosing_def_to_moniker (function)
macro_rules! Depcrate_monikerenclosing_def_to_moniker {
() => {
// Module: crate::moniker
// Provides: {"enclosing_def_to_moniker"}
// Dependencies: {}
fn enclosing_def_to_moniker (db : & RootDatabase , mut def : Definition , from_crate : Crate ,) -> Option < Moniker > { loop { let enclosing_def = def . enclosing_definition (db) ? ; if let Some (enclosing_moniker) = def_to_non_local_moniker (db , enclosing_def , from_crate) { return Some (enclosing_moniker) ; } def = enclosing_def ; } }
};
}
