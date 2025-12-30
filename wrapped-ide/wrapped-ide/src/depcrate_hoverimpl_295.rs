// Generated macro for impl_295 (impl)
macro_rules! Depcrate_hoverimpl_295 {
() => {
// Module: crate::hover
// Provides: {"impl_295"}
// Dependencies: {}
impl HoverAction { fn goto_type_from_targets (db : & RootDatabase , targets : Vec < hir :: ModuleDef > , edition : Edition ,) -> Option < Self > { let targets = targets . into_iter () . filter_map (| it | { Some (HoverGotoTypeData { mod_path : render :: path (db , it . module (db) ? , it . name (db) . map (| name | name . display (db , edition) . to_string ()) , edition ,) , nav : it . try_to_nav (db) ? . call_site () , }) }) . collect :: < Vec < _ > > () ; targets . is_empty () . not () . then_some (HoverAction :: GoToType (targets)) } }
};
}
