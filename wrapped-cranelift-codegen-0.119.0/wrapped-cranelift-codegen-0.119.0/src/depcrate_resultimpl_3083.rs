// Generated macro for impl_3083 (impl)
macro_rules! Depcrate_resultimpl_3083 {
() => {
// Module: crate::result
// Provides: {"impl_3083"}
// Dependencies: {}
impl std :: error :: Error for CodegenError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { CodegenError :: Verifier (source) => Some (source) , CodegenError :: ImplLimitExceeded { .. } | CodegenError :: CodeTooLarge { .. } | CodegenError :: Unsupported { .. } => None , # [cfg (feature = "unwind")] CodegenError :: RegisterMappingError { .. } => None , CodegenError :: Regalloc (..) => None , CodegenError :: Pcc (..) => None , } } }
};
}
