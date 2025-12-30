// Generated macro for impl_277 (impl)
macro_rules! Depcrate_ast_traitsimpl_277 {
() => {
// Module: crate::ast_traits
// Provides: {"impl_277"}
// Dependencies: {}
impl HasTokens for StmtKind { fn tokens (& self) -> Option < & LazyAttrTokenStream > { match self { StmtKind :: Let (local) => local . tokens . as_ref () , StmtKind :: Item (item) => item . tokens () , StmtKind :: Expr (expr) | StmtKind :: Semi (expr) => expr . tokens () , StmtKind :: Empty => None , StmtKind :: MacCall (mac) => mac . tokens . as_ref () , } } fn tokens_mut (& mut self) -> Option < & mut Option < LazyAttrTokenStream > > { match self { StmtKind :: Let (local) => Some (& mut local . tokens) , StmtKind :: Item (item) => item . tokens_mut () , StmtKind :: Expr (expr) | StmtKind :: Semi (expr) => expr . tokens_mut () , StmtKind :: Empty => None , StmtKind :: MacCall (mac) => Some (& mut mac . tokens) , } } }
};
}
