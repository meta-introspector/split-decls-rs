// Generated macro for impl_364 (impl)
macro_rules! Depcrate_consteval_tests_method_resolutionimpl_364 {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"impl_364"}
// Dependencies: {}
impl MethodResolutionUnstableFeatures { pub fn from_def_map (def_map : & DefMap) -> Self { Self { arbitrary_self_types : def_map . is_unstable_feature_enabled (& sym :: arbitrary_self_types) , arbitrary_self_types_pointers : def_map . is_unstable_feature_enabled (& sym :: arbitrary_self_types_pointers) , supertrait_item_shadowing : def_map . is_unstable_feature_enabled (& sym :: supertrait_item_shadowing) , } } }
};
}
