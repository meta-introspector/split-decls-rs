// Generated macro for AstIdLoc (trait)
macro_rules! DepcrateAstIdLoc {
() => {
// Module: crate
// Provides: {"AstIdLoc"}
// Dependencies: {}
pub trait AstIdLoc { type Container ; type Ast : AstNode ; fn ast_id (& self) -> AstId < Self :: Ast > ; fn container (& self) -> Self :: Container ; }
};
}
