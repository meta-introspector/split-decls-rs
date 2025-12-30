// Generated macro for impl_63 (impl)
macro_rules! Depcrate_wnafimpl_63 {
() => {
// Module: crate::wnaf
// Provides: {"impl_63"}
// Dependencies: {}
# [cfg (feature = "wnaf-memuse")] impl < 'a , G : Group + memuse :: DynamicUsage > memuse :: DynamicUsage for Wnaf < usize , Vec < G > , & 'a [i64] > { fn dynamic_usage (& self) -> usize { self . base . dynamic_usage () } fn dynamic_usage_bounds (& self) -> (usize , Option < usize >) { self . base . dynamic_usage_bounds () } }
};
}
