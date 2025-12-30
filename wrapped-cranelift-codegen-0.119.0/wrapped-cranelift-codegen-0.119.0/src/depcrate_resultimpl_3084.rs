// Generated macro for impl_3084 (impl)
macro_rules! Depcrate_resultimpl_3084 {
() => {
// Module: crate::result
// Provides: {"impl_3084"}
// Dependencies: {}
impl std :: fmt :: Display for CodegenError { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { match self { CodegenError :: Verifier (_) => write ! (f , "Verifier errors") , CodegenError :: ImplLimitExceeded => write ! (f , "Implementation limit exceeded") , CodegenError :: CodeTooLarge => write ! (f , "Code for function is too large") , CodegenError :: Unsupported (feature) => write ! (f , "Unsupported feature: {feature}") , # [cfg (feature = "unwind")] CodegenError :: RegisterMappingError (_0) => write ! (f , "Register mapping error") , CodegenError :: Regalloc (errors) => write ! (f , "Regalloc validation errors: {errors:?}") , CodegenError :: Pcc (e) => write ! (f , "Proof-carrying-code validation error: {e:?}") , } } }
};
}
