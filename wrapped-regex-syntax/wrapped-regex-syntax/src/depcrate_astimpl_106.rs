// Generated macro for impl_106 (impl)
macro_rules! Depcrate_astimpl_106 {
() => {
// Module: crate::ast
// Provides: {"impl_106"}
// Dependencies: {}
impl ClassSetItem { # [doc = " Return the span of this character class set item."] pub fn span (& self) -> & Span { match * self { ClassSetItem :: Empty (ref span) => span , ClassSetItem :: Literal (ref x) => & x . span , ClassSetItem :: Range (ref x) => & x . span , ClassSetItem :: Ascii (ref x) => & x . span , ClassSetItem :: Perl (ref x) => & x . span , ClassSetItem :: Unicode (ref x) => & x . span , ClassSetItem :: Bracketed (ref x) => & x . span , ClassSetItem :: Union (ref x) => & x . span , } } }
};
}
