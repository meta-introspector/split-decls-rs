// Generated macro for try_lookup_macro_def_in_macro_use (function)
macro_rules! Depcrate_goto_definitiontry_lookup_macro_def_in_macro_use {
() => {
// Module: crate::goto_definition
// Provides: {"try_lookup_macro_def_in_macro_use"}
// Dependencies: {}
fn try_lookup_macro_def_in_macro_use (sema : & Semantics < '_ , RootDatabase > , token : SyntaxToken ,) -> Option < NavigationTarget > { let extern_crate = token . parent () ? . ancestors () . find_map (ast :: ExternCrate :: cast) ? ; let extern_crate = sema . to_def (& extern_crate) ? ; let krate = extern_crate . resolved_crate (sema . db) ? ; for mod_def in krate . root_module () . declarations (sema . db) { if let ModuleDef :: Macro (mac) = mod_def && mac . name (sema . db) . as_str () == token . text () && let Some (nav) = mac . try_to_nav (sema) { return Some (nav . call_site) ; } } None }
};
}
