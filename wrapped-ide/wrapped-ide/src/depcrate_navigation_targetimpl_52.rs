// Generated macro for impl_52 (impl)
macro_rules! Depcrate_navigation_targetimpl_52 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_52"}
// Dependencies: {}
impl TryToNav for hir :: Impl { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { let db = sema . db ; let InFile { file_id , value } = self . source (db) ? ; let derive_path = self . as_builtin_derive_path (db) ; let (file_id , focus , syntax) = match & derive_path { Some (attr) => (attr . file_id . into () , None , attr . value . syntax ()) , None => (file_id , value . self_ty () , value . syntax ()) , } ; Some (orig_range_with_focus (db , file_id , syntax , focus) . map (| (FileRange { file_id , range : full_range } , focus_range) | { NavigationTarget :: from_syntax (file_id , sym :: kw_impl , focus_range , full_range , SymbolKind :: Impl ,) } ,)) } }
};
}
