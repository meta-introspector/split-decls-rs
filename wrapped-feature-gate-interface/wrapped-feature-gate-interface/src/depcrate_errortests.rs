// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { super :: FeatureGateError , strum :: IntoEnumIterator } ; # [test] fn test_system_error_from_primitive_exhaustive () { for variant in FeatureGateError :: iter () { let variant_u32 = variant . clone () as u32 ; assert_eq ! (FeatureGateError :: from_repr (variant_u32) . unwrap () , variant) ; assert_eq ! (FeatureGateError :: try_from (variant_u32) . unwrap () , variant) ; } } }
};
}
