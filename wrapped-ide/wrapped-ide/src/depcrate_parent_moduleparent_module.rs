// Generated macro for parent_module (function)
macro_rules! Depcrate_parent_moduleparent_module {
() => {
// Module: crate::parent_module
// Provides: {"parent_module"}
// Dependencies: {}
# [doc = " This returns `Vec` because a module may be included from several places."] pub (crate) fn parent_module (db : & RootDatabase , position : FilePosition) -> Vec < NavigationTarget > { let sema = Semantics :: new (db) ; let source_file = sema . parse_guess_edition (position . file_id) ; let mut module = find_node_at_offset :: < ast :: Module > (source_file . syntax () , position . offset) ; if let Some (m) = & module && ! m . item_list () . is_some_and (| it | it . syntax () . text_range () . contains_inclusive (position . offset)) { cov_mark :: hit ! (test_resolve_parent_module_on_module_decl) ; module = m . syntax () . ancestors () . skip (1) . find_map (ast :: Module :: cast) ; } match module { Some (module) => sema . to_def (& module) . into_iter () . flat_map (| module | NavigationTarget :: from_module_to_decl (db , module)) . collect () , None => sema . file_to_module_defs (position . file_id) . flat_map (| module | NavigationTarget :: from_module_to_decl (db , module)) . collect () , } }
};
}
