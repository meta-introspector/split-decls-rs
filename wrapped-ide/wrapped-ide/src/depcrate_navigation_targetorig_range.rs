// Generated macro for orig_range (function)
macro_rules! Depcrate_navigation_targetorig_range {
() => {
// Module: crate::navigation_target
// Provides: {"orig_range"}
// Dependencies: {}
fn orig_range (db : & RootDatabase , hir_file : HirFileId , value : & SyntaxNode ,) -> UpmappingResult < (FileRange , Option < TextRange >) > { UpmappingResult { call_site : (InFile :: new (hir_file , value) . original_file_range_rooted (db) . into_file_id (db) , None ,) , def_site : None , } }
};
}
