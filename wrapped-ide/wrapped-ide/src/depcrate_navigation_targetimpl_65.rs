// Generated macro for impl_65 (impl)
macro_rules! Depcrate_navigation_targetimpl_65 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_65"}
// Dependencies: {}
impl TryToNav for hir :: ConstParam { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { let db = sema . db ; let InFile { file_id , value } = self . merge () . source (db) ? ; let name = self . name (db) . symbol () . clone () ; let value = match value { Either :: Left (ast :: TypeOrConstParam :: Const (x)) => x , _ => { never ! () ; return None ; } } ; Some (orig_range_with_focus (db , file_id , value . syntax () , value . name ()) . map (| (FileRange { file_id , range : full_range } , focus_range) | NavigationTarget { file_id , name : name . clone () , alias : None , kind : Some (SymbolKind :: ConstParam) , full_range , focus_range , container_name : None , description : None , docs : None , } ,)) } }
};
}
