// Generated macro for is_test_fn (function)
macro_rules! Depcrate_render_testis_test_fn {
() => {
// Module: crate::render::test
// Provides: {"is_test_fn"}
// Dependencies: {}
fn is_test_fn (item_fn : & ItemFn) -> bool { item_fn . has_attr_that_ends_with (& parse_quote ! { test }) }
};
}
