// Generated macro for find_items (function)
macro_rules! Depcrate_items_locatorfind_items {
() => {
// Module: crate::items_locator
// Provides: {"find_items"}
// Dependencies: {}
fn find_items (db : & RootDatabase , krate : Crate , local_query : symbol_index :: Query , external_query : import_map :: Query ,) -> impl Iterator < Item = (ItemInNs , Complete) > { let _p = tracing :: info_span ! ("find_items") . entered () ; let external_importables = krate . query_external_importables (db , external_query) . map (| (external_importable , do_not_complete) | { let external_importable = match external_importable { Either :: Left (module_def) => ItemInNs :: from (module_def) , Either :: Right (macro_def) => ItemInNs :: from (macro_def) , } ; (external_importable , do_not_complete) } ,) ; let mut local_results = Vec :: new () ; local_query . search (& symbol_index :: crate_symbols (db , krate) , | local_candidate | { let def = match local_candidate . def { hir :: ModuleDef :: Macro (macro_def) => ItemInNs :: Macros (macro_def) , def => ItemInNs :: from (def) , } ; local_results . push ((def , local_candidate . do_not_complete)) ; ControlFlow :: < () > :: Continue (()) }) ; local_results . into_iter () . chain (external_importables) }
};
}
