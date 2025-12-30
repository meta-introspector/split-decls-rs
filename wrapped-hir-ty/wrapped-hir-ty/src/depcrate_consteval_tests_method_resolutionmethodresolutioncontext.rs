// Generated macro for MethodResolutionContext (struct)
macro_rules! Depcrate_consteval_tests_method_resolutionMethodResolutionContext {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"MethodResolutionContext"}
// Dependencies: {}
pub struct MethodResolutionContext < 'a , 'db > { pub infcx : & 'a InferCtxt < 'db > , pub resolver : & 'a Resolver < 'db > , pub env : & 'a TraitEnvironment < 'db > , pub traits_in_scope : & 'a FxHashSet < TraitId > , pub edition : Edition , pub unstable_features : & 'a MethodResolutionUnstableFeatures , }
};
}
