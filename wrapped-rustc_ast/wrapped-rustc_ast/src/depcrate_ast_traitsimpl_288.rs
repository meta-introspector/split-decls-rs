// Generated macro for impl_288 (impl)
macro_rules! Depcrate_ast_traitsimpl_288 {
() => {
// Module: crate::ast_traits
// Provides: {"impl_288"}
// Dependencies: {}
impl HasAttrs for StmtKind { const SUPPORTS_CUSTOM_INNER_ATTRS : bool = true ; fn attrs (& self) -> & [Attribute] { match self { StmtKind :: Let (local) => & local . attrs , StmtKind :: Expr (expr) | StmtKind :: Semi (expr) => expr . attrs () , StmtKind :: Item (item) => item . attrs () , StmtKind :: Empty => & [] , StmtKind :: MacCall (mac) => & mac . attrs , } } fn visit_attrs (& mut self , f : impl FnOnce (& mut AttrVec)) { match self { StmtKind :: Let (local) => f (& mut local . attrs) , StmtKind :: Expr (expr) | StmtKind :: Semi (expr) => expr . visit_attrs (f) , StmtKind :: Item (item) => item . visit_attrs (f) , StmtKind :: Empty => { } StmtKind :: MacCall (mac) => f (& mut mac . attrs) , } } }
};
}
