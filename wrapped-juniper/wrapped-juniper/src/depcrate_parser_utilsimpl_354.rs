// Generated macro for impl_354 (impl)
macro_rules! Depcrate_parser_utilsimpl_354 {
() => {
// Module: crate::parser::utils
// Provides: {"impl_354"}
// Dependencies: {}
impl Span { # [doc (hidden)] # [inline] pub fn zero_width (pos : SourcePosition) -> Self { Self { start : pos , end : pos , } } # [doc (hidden)] # [inline] pub fn single_width (pos : SourcePosition) -> Self { let mut end = pos ; end . advance_col () ; Self { start : pos , end } } # [doc (hidden)] # [inline] pub fn unlocated () -> Self { Self { start : SourcePosition :: new_origin () , end : SourcePosition :: new_origin () , } } }
};
}
