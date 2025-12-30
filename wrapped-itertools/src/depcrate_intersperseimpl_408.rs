// Generated macro for impl_408 (impl)
macro_rules! Depcrate_intersperseimpl_408 {
() => {
// Module: crate::intersperse
// Provides: {"impl_408"}
// Dependencies: {}
impl < Item , F : FnMut () -> Item > IntersperseElement < Item > for F { fn generate (& mut self) -> Item { self () } }
};
}
