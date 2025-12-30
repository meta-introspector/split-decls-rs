// Generated macro for impl_31 (impl)
macro_rules! Depcrate_v0impl_31 {
() => {
// Module: crate::v0
// Provides: {"impl_31"}
// Dependencies: {}
impl ParseError { # [doc = " Snippet to print when the error is initially encountered."] fn message (& self) -> & str { match self { ParseError :: Invalid => "{invalid syntax}" , ParseError :: RecursedTooDeep => "{recursion limit reached}" , } } }
};
}
