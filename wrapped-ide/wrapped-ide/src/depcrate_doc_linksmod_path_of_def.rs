// Generated macro for mod_path_of_def (function)
macro_rules! Depcrate_doc_linksmod_path_of_def {
() => {
// Module: crate::doc_links
// Provides: {"mod_path_of_def"}
// Dependencies: {}
fn mod_path_of_def (db : & RootDatabase , def : Definition) -> Option < String > { def . canonical_module_path (db) . map (| it | { let mut path = String :: new () ; it . flat_map (| it | it . name (db)) . for_each (| name | format_to ! (path , "{}/" , name . as_str ())) ; path }) }
};
}
