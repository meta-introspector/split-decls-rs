// Generated macro for orig_range_r (function)
macro_rules! Depcrate_navigation_targetorig_range_r {
() => {
// Module: crate::navigation_target
// Provides: {"orig_range_r"}
// Dependencies: {}
fn orig_range_r (db : & RootDatabase , hir_file : HirFileId , value : TextRange ,) -> UpmappingResult < (FileRange , Option < TextRange >) > { UpmappingResult { call_site : (InFile :: new (hir_file , value) . original_node_file_range (db) . 0 . into_file_id (db) , None ,) , def_site : None , } }
};
}
