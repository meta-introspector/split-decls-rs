// Generated macro for impl_400 (impl)
macro_rules! Depcrate_redactionimpl_400 {
() => {
// Module: crate::redaction
// Provides: {"impl_400"}
// Dependencies: {}
impl SelectorParseError { # [doc = " Return the column of where the error occurred."] pub fn column (& self) -> usize { match self . 0 . line_col { pest :: error :: LineColLocation :: Pos ((_ , col)) => col , pest :: error :: LineColLocation :: Span ((_ , col) , _) => col , } } }
};
}
