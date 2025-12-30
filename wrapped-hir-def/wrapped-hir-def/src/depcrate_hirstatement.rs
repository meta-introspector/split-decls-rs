// Generated macro for Statement (enum)
macro_rules! Depcrate_hirStatement {
() => {
// Module: crate::hir
// Provides: {"Statement"}
// Dependencies: {}
# [derive (Debug , Clone , Eq , PartialEq)] pub enum Statement { Let { pat : PatId , type_ref : Option < TypeRefId > , initializer : Option < ExprId > , else_branch : Option < ExprId > , } , Expr { expr : ExprId , has_semi : bool , } , Item (Item) , }
};
}
