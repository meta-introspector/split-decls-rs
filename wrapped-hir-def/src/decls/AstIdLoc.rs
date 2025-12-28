macro_rules! AstIdLoc {
    () => {
        pub trait AstIdLoc { type Container ; type Ast : AstNode ; fn ast_id (& self) -> AstId < Self :: Ast > ; fn container (& self) -> Self :: Container ; }
    };
}

AstIdLoc!();