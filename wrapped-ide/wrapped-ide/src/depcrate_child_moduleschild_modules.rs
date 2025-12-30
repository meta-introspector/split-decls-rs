// Generated macro for child_modules (function)
macro_rules! Depcrate_child_moduleschild_modules {
() => {
// Module: crate::child_modules
// Provides: {"child_modules"}
// Dependencies: {}
# [doc = " This returns `Vec` because a module may be included from several places."] pub (crate) fn child_modules (db : & RootDatabase , position : FilePosition) -> Vec < NavigationTarget > { let sema = Semantics :: new (db) ; let source_file = sema . parse_guess_edition (position . file_id) ; let module = find_node_at_offset :: < ast :: Module > (source_file . syntax () , position . offset) ; match module { Some (module) => { sema . to_def (& module) . into_iter () . flat_map (| module | module . children (db)) . map (| module | NavigationTarget :: from_module_to_decl (db , module) . call_site ()) . collect () } None => { sema . file_to_module_defs (position . file_id) . flat_map (| module | module . children (db)) . map (| module | NavigationTarget :: from_module_to_decl (db , module) . call_site ()) . collect () } } }
};
}
