// Generated macro for impl_35 (impl)
macro_rules! Depcrate_cursorimpl_35 {
() => {
// Module: crate::cursor
// Provides: {"impl_35"}
// Dependencies: {}
impl fmt :: Debug for SyntaxNode { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SyntaxNode") . field ("kind" , & self . kind ()) . field ("text_range" , & self . text_range ()) . finish () } }
};
}
