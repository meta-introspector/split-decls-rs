// Generated macro for impl_232 (impl)
macro_rules! Depcrate_builder_rangeimpl_232 {
() => {
// Module: crate::builder::range
// Provides: {"impl_232"}
// Dependencies: {}
impl std :: ops :: RangeBounds < usize > for ValueRange { fn start_bound (& self) -> std :: ops :: Bound < & usize > { std :: ops :: Bound :: Included (& self . start_inclusive) } fn end_bound (& self) -> std :: ops :: Bound < & usize > { std :: ops :: Bound :: Included (& self . end_inclusive) } }
};
}
