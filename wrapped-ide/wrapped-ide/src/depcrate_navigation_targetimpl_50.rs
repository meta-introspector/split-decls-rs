// Generated macro for impl_50 (impl)
macro_rules! Depcrate_navigation_targetimpl_50 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_50"}
// Dependencies: {}
impl ToNav for hir :: Module { fn to_nav (& self , db : & RootDatabase) -> UpmappingResult < NavigationTarget > { let InFile { file_id , value } = self . definition_source (db) ; let name = self . name (db) . map (| it | it . symbol () . clone ()) . unwrap_or_else (| | sym :: underscore) ; let (syntax , focus) = match & value { ModuleSource :: SourceFile (node) => (node . syntax () , None) , ModuleSource :: Module (node) => (node . syntax () , node . name ()) , ModuleSource :: BlockExpr (node) => (node . syntax () , None) , } ; orig_range_with_focus (db , file_id , syntax , focus) . map (| (FileRange { file_id , range : full_range } , focus_range) | { NavigationTarget :: from_syntax (file_id , name . clone () , focus_range , full_range , SymbolKind :: Module ,) } ,) } }
};
}
