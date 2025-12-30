// Generated macro for impl_130 (impl)
macro_rules! Depcrate_astimpl_130 {
() => {
// Module: crate::ast
// Provides: {"impl_130"}
// Dependencies: {}
impl DelimArgs { # [doc = " Whether a macro with these arguments needs a semicolon"] # [doc = " when used as a standalone item or statement."] pub fn need_semicolon (& self) -> bool { ! matches ! (self , DelimArgs { delim : Delimiter :: Brace , .. }) } }
};
}
