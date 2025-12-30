// Generated macro for impl_100 (impl)
macro_rules! Depcrate_syntax_textimpl_100 {
() => {
// Module: crate::syntax_text
// Provides: {"impl_100"}
// Dependencies: {}
impl fmt :: Display for SyntaxText { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . try_for_each_chunk (| chunk | fmt :: Display :: fmt (chunk , f)) } }
};
}
