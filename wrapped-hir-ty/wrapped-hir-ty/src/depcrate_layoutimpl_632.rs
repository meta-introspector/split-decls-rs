// Generated macro for impl_632 (impl)
macro_rules! Depcrate_layoutimpl_632 {
() => {
// Module: crate::layout
// Provides: {"impl_632"}
// Dependencies: {}
impl fmt :: Display for LayoutError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { LayoutError :: BadCalc (err) => err . fallback_fmt (f) , LayoutError :: HasErrorConst => write ! (f , "type contains an unevaluatable const") , LayoutError :: HasErrorType => write ! (f , "type contains an error") , LayoutError :: HasPlaceholder => write ! (f , "type contains placeholders") , LayoutError :: InvalidSimdType => write ! (f , "invalid simd type definition") , LayoutError :: NotImplemented => write ! (f , "not implemented") , LayoutError :: RecursiveTypeWithoutIndirection => { write ! (f , "recursive type without indirection") } LayoutError :: TargetLayoutNotAvailable => write ! (f , "target layout not available") , LayoutError :: Unknown => write ! (f , "unknown") , LayoutError :: UserReprTooSmall => { write ! (f , "the `#[repr]` hint is too small to hold the discriminants of the enum") } } } }
};
}
