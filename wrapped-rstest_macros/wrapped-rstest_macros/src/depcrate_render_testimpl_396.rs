// Generated macro for impl_396 (impl)
macro_rules! Depcrate_render_testimpl_396 {
() => {
// Module: crate::render::test
// Provides: {"impl_396"}
// Dependencies: {}
impl ModuleInspector for ItemMod { fn get_tests (& self) -> Vec < ItemFn > { self . content . as_ref () . map (| (_ , items) | { items . iter () . filter_map (| it | match it { syn :: Item :: Fn (item_fn) if is_test_fn (item_fn) => Some (item_fn . clone ()) , _ => None , }) . collect () }) . unwrap_or_default () } fn get_all_tests (& self) -> Vec < ItemFn > { let mut f = TestFunctions (vec ! []) ; f . visit_item_mod (& self) ; f . 0 } fn get_modules (& self) -> Vec < ItemMod > { self . content . as_ref () . map (| (_ , items) | { items . iter () . filter_map (| it | match it { syn :: Item :: Mod (item_mod) => Some (item_mod . clone ()) , _ => None , }) . collect () }) . unwrap_or_default () } }
};
}
