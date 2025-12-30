// Generated macro for impl_66 (impl)
macro_rules! Depcrate_navigation_targetimpl_66 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_66"}
// Dependencies: {}
impl TryToNav for hir :: InlineAsmOperand { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { let db = sema . db ; let InFile { file_id , value } = & self . source (db) ? ; let file_id = * file_id ; Some (orig_range_with_focus (db , file_id , value . syntax () , value . name ()) . map (| (FileRange { file_id , range : full_range } , focus_range) | NavigationTarget { file_id , name : self . name (db) . map_or_else (| | sym :: underscore . clone () , | it | it . symbol () . clone ()) , alias : None , kind : Some (SymbolKind :: Local) , full_range , focus_range , container_name : None , description : None , docs : None , } ,)) } }
};
}
