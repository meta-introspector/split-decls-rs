// Generated macro for MethodCallee (struct)
macro_rules! Depcrate_consteval_tests_method_resolutionMethodCallee {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"MethodCallee"}
// Dependencies: {}
# [derive (Clone , Copy , Debug)] pub (crate) struct MethodCallee < 'db > { # [doc = " Impl method ID, for inherent methods, or trait method ID, otherwise."] pub def_id : FunctionId , pub args : GenericArgs < 'db > , # [doc = " Instantiated method signature, i.e., it has been"] # [doc = " instantiated, normalized, and has had late-bound"] # [doc = " lifetimes replaced with inference variables."] pub sig : FnSig < 'db > , }
};
}
