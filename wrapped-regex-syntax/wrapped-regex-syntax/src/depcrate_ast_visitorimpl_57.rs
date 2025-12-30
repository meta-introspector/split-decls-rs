// Generated macro for impl_57 (impl)
macro_rules! Depcrate_ast_visitorimpl_57 {
() => {
// Module: crate::ast::visitor
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'a > ClassInduct < 'a > { fn from_bracketed (ast : & 'a ast :: ClassBracketed) -> ClassInduct < 'a > { ClassInduct :: from_set (& ast . kind) } fn from_set (ast : & 'a ast :: ClassSet) -> ClassInduct < 'a > { match * ast { ast :: ClassSet :: Item (ref item) => ClassInduct :: Item (item) , ast :: ClassSet :: BinaryOp (ref op) => ClassInduct :: BinaryOp (op) , } } }
};
}
