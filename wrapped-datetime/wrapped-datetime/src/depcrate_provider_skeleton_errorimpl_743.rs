// Generated macro for impl_743 (impl)
macro_rules! Depcrate_provider_skeleton_errorimpl_743 {
() => {
// Module: crate::provider::skeleton::error
// Provides: {"impl_743"}
// Dependencies: {}
impl From < fields :: SymbolError > for SkeletonError { fn from (symbol_error : fields :: SymbolError) -> Self { match symbol_error { fields :: SymbolError :: Invalid (ch) => match ch { b'-' => Self :: SkeletonHasVariant , _ => Self :: SymbolInvalid (ch) , } , fields :: SymbolError :: InvalidIndex (_) => unimplemented ! () , fields :: SymbolError :: Unknown (ch) => { match ch { 'B' | 'Q' | 'q' | 'Y' | 'w' | 'W' => Self :: SymbolUnimplemented (ch) , _ => Self :: SymbolUnknown (ch) , } } } } }
};
}
