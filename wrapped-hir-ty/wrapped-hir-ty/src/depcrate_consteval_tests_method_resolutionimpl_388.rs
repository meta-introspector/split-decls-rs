// Generated macro for impl_388 (impl)
macro_rules! Depcrate_consteval_tests_method_resolutionimpl_388 {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"impl_388"}
// Dependencies: {}
impl OneTraitImplsBuilder { fn finish (self) -> OneTraitImpls { let mut non_blanket_impls = self . non_blanket_impls . into_iter () . map (| (self_ty , impls) | (self_ty , impls . into_boxed_slice ())) . collect :: < FxHashMap < _ , _ > > () ; non_blanket_impls . shrink_to_fit () ; let blanket_impls = self . blanket_impls . into_boxed_slice () ; OneTraitImpls { non_blanket_impls , blanket_impls } } }
};
}
