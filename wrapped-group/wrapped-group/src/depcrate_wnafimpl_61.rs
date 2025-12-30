// Generated macro for impl_61 (impl)
macro_rules! Depcrate_wnafimpl_61 {
() => {
// Module: crate::wnaf
// Provides: {"impl_61"}
// Dependencies: {}
# [cfg (feature = "wnaf-memuse")] impl < 'a , G : Group > memuse :: DynamicUsage for Wnaf < usize , & 'a [G] , Vec < i64 > > { fn dynamic_usage (& self) -> usize { self . scalar . dynamic_usage () } fn dynamic_usage_bounds (& self) -> (usize , Option < usize >) { self . scalar . dynamic_usage_bounds () } }
};
}
