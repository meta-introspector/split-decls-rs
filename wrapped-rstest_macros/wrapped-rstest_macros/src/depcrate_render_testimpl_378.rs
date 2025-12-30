// Generated macro for impl_378 (impl)
macro_rules! Depcrate_render_testimpl_378 {
() => {
// Module: crate::render::test
// Provides: {"impl_378"}
// Dependencies: {}
impl SetAsync for ItemFn { fn set_async (& mut self , is_async : bool) { self . sig . asyncness = if is_async { Some (parse_quote ! { async }) } else { None } ; } }
};
}
