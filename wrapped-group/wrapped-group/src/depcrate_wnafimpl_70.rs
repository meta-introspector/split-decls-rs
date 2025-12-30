// Generated macro for impl_70 (impl)
macro_rules! Depcrate_wnafimpl_70 {
() => {
// Module: crate::wnaf
// Provides: {"impl_70"}
// Dependencies: {}
# [cfg (feature = "wnaf-memuse")] impl < G : Group + memuse :: DynamicUsage , const WINDOW_SIZE : usize > memuse :: DynamicUsage for WnafBase < G , WINDOW_SIZE > { fn dynamic_usage (& self) -> usize { self . table . dynamic_usage () } fn dynamic_usage_bounds (& self) -> (usize , Option < usize >) { self . table . dynamic_usage_bounds () } }
};
}
