// Generated macro for impl_384 (impl)
macro_rules! Depcrate_render_testimpl_384 {
() => {
// Module: crate::render::test
// Provides: {"impl_384"}
// Dependencies: {}
impl QueryAttrs for ItemFn { fn has_attr (& self , attr : & syn :: Path) -> bool { self . attrs . iter () . find (| a | a . path () == attr) . is_some () } fn has_attr_that_ends_with (& self , name : & syn :: PathSegment) -> bool { self . attrs . iter () . find (| a | attr_ends_with (a , name)) . is_some () } }
};
}
