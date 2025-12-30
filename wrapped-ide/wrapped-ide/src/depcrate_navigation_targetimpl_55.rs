// Generated macro for impl_55 (impl)
macro_rules! Depcrate_navigation_targetimpl_55 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_55"}
// Dependencies: {}
impl TryToNav for hir :: Macro { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { let db = sema . db ; let src = self . source (db) ? ; let name_owner : & dyn ast :: HasName = match & src . value { Either :: Left (it) => it , Either :: Right (it) => it , } ; Some (NavigationTarget :: from_named (db , src . as_ref () . with_value (name_owner) , self . kind (db) . into () ,) . map (| mut res | { res . docs = self . docs (db) ; res }) ,) } }
};
}
