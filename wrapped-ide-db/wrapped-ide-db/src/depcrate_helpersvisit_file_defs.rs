// Generated macro for visit_file_defs (function)
macro_rules! Depcrate_helpersvisit_file_defs {
() => {
// Module: crate::helpers
// Provides: {"visit_file_defs"}
// Dependencies: {}
# [doc = " Iterates all `ModuleDef`s and `Impl` blocks of the given file."] pub fn visit_file_defs (sema : & Semantics < '_ , RootDatabase > , file_id : FileId , cb : & mut dyn FnMut (Definition) ,) { let db = sema . db ; let module = match sema . file_to_module_def (file_id) { Some (it) => it , None => return , } ; let mut defs : VecDeque < _ > = module . declarations (db) . into () ; while let Some (def) = defs . pop_front () { if let ModuleDef :: Module (submodule) = def && submodule . is_inline (db) { defs . extend (submodule . declarations (db)) ; submodule . impl_defs (db) . into_iter () . for_each (| impl_ | cb (impl_ . into ())) ; } cb (def . into ()) ; } module . impl_defs (db) . into_iter () . for_each (| impl_ | cb (impl_ . into ())) ; let is_root = module . is_crate_root () ; module . legacy_macros (db) . into_iter () . filter (| it | ! (is_root && it . is_macro_export (db))) . for_each (| mac | cb (mac . into ())) ; }
};
}
