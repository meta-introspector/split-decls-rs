// Generated macro for impl_130 (impl)
macro_rules! Depcrateimpl_130 {
() => {
// Module: crate
// Provides: {"impl_130"}
// Dependencies: {}
# [allow (clippy :: derive_ord_xor_partial_ord)] impl < T : FloatCore > Ord for NotNan < T > { fn cmp (& self , other : & NotNan < T >) -> Ordering { self . partial_cmp (other) . expect ("partial_cmp failed for non-NaN value") } }
};
}
