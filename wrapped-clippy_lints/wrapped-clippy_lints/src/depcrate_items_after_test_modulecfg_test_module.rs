// Generated macro for cfg_test_module (function)
macro_rules! Depcrate_items_after_test_modulecfg_test_module {
() => {
// Module: crate::items_after_test_module
// Provides: {"cfg_test_module"}
// Dependencies: {}
fn cfg_test_module < 'tcx > (cx : & LateContext < 'tcx > , item : & Item < 'tcx >) -> bool { if let ItemKind :: Mod (_ , test_mod) = item . kind && item . span . hi () == test_mod . spans . inner_span . hi () && is_cfg_test (cx . tcx , item . hir_id ()) && ! item . span . from_expansion () && ! is_from_proc_macro (cx , item) { true } else { false } }
};
}
