// Generated macro for impl_55 (impl)
macro_rules! Depcrate_ast_visitorimpl_55 {
() => {
// Module: crate::ast::visitor
// Provides: {"impl_55"}
// Dependencies: {}
impl < 'a > Frame < 'a > { # [doc = " Perform the next inductive step on this frame and return the next"] # [doc = " child AST node to visit."] fn child (& self) -> & 'a Ast { match * self { Frame :: Repetition (rep) => & rep . ast , Frame :: Group (group) => & group . ast , Frame :: Concat { head , .. } => head , Frame :: Alternation { head , .. } => head , } } }
};
}
