// Generated macro for impl_388 (impl)
macro_rules! Depcrate_render_testimpl_388 {
() => {
// Module: crate::render::test
// Provides: {"impl_388"}
// Dependencies: {}
impl < 'ast > Visit < 'ast > for TestFunctions { fn visit_item_fn (& mut self , item_fn : & 'ast ItemFn) { if Self :: is_test_fn (item_fn) { self . 0 . push (item_fn . clone ()) } } }
};
}
