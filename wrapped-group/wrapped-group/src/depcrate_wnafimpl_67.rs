// Generated macro for impl_67 (impl)
macro_rules! Depcrate_wnafimpl_67 {
() => {
// Module: crate::wnaf
// Provides: {"impl_67"}
// Dependencies: {}
# [cfg (feature = "wnaf-memuse")] impl < F : PrimeField , const WINDOW_SIZE : usize > memuse :: DynamicUsage for WnafScalar < F , WINDOW_SIZE > { fn dynamic_usage (& self) -> usize { self . wnaf . dynamic_usage () } fn dynamic_usage_bounds (& self) -> (usize , Option < usize >) { self . wnaf . dynamic_usage_bounds () } }
};
}
