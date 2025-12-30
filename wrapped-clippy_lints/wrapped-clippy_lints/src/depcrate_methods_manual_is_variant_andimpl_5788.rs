// Generated macro for impl_5788 (impl)
macro_rules! Depcrate_methods_manual_is_variant_andimpl_5788 {
() => {
// Module: crate::methods::manual_is_variant_and
// Provides: {"impl_5788"}
// Dependencies: {}
impl Flavor { const fn symbol (self) -> Symbol { match self { Self :: Option => sym :: Option , Self :: Result => sym :: Result , } } const fn positive (self) -> Symbol { match self { Self :: Option => sym :: Some , Self :: Result => sym :: Ok , } } }
};
}
