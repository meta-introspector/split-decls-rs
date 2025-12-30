// Generated macro for impl_58 (impl)
macro_rules! Depcrate_ast_visitorimpl_58 {
() => {
// Module: crate::ast::visitor
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a > core :: fmt :: Debug for ClassFrame < 'a > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let x = match * self { ClassFrame :: Union { .. } => "Union" , ClassFrame :: Binary { .. } => "Binary" , ClassFrame :: BinaryLHS { .. } => "BinaryLHS" , ClassFrame :: BinaryRHS { .. } => "BinaryRHS" , } ; write ! (f , "{x}") } }
};
}
