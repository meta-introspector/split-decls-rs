// Generated macro for impl_3072 (impl)
macro_rules! Depcrate_remove_constant_phisimpl_3072 {
() => {
// Module: crate::remove_constant_phis
// Provides: {"impl_3072"}
// Dependencies: {}
impl < 'a > BlockSummary < 'a > { # [doc = " Construct a new `BlockSummary`, using `values` as its backing storage."] # [inline] fn new (bump : & 'a Bump , formals : & [Value]) -> Self { Self { formals : bump . alloc_slice_copy (formals) , dests : Default :: default () , } } }
};
}
