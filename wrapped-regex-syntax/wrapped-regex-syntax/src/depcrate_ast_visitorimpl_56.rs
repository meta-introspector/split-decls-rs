// Generated macro for impl_56 (impl)
macro_rules! Depcrate_ast_visitorimpl_56 {
() => {
// Module: crate::ast::visitor
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'a > ClassFrame < 'a > { # [doc = " Perform the next inductive step on this frame and return the next"] # [doc = " child class node to visit."] fn child (& self) -> ClassInduct < 'a > { match * self { ClassFrame :: Union { head , .. } => ClassInduct :: Item (head) , ClassFrame :: Binary { op , .. } => ClassInduct :: BinaryOp (op) , ClassFrame :: BinaryLHS { ref lhs , .. } => { ClassInduct :: from_set (lhs) } ClassFrame :: BinaryRHS { ref rhs , .. } => { ClassInduct :: from_set (rhs) } } } }
};
}
