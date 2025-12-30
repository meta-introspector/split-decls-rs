// Generated macro for eq_stmt (function)
macro_rules! Depcrate_ast_utilseq_stmt {
() => {
// Module: crate::ast_utils
// Provides: {"eq_stmt"}
// Dependencies: {}
pub fn eq_stmt (l : & Stmt , r : & Stmt) -> bool { use StmtKind :: * ; match (& l . kind , & r . kind) { (Let (l) , Let (r)) => { eq_pat (& l . pat , & r . pat) && both (l . ty . as_ref () , r . ty . as_ref () , | l , r | eq_ty (l , r)) && eq_local_kind (& l . kind , & r . kind) && over (& l . attrs , & r . attrs , eq_attr) } , (Item (l) , Item (r)) => eq_item (l , r , eq_item_kind) , (Expr (l) , Expr (r)) | (Semi (l) , Semi (r)) => eq_expr (l , r) , (Empty , Empty) => true , (MacCall (l) , MacCall (r)) => { l . style == r . style && eq_mac_call (& l . mac , & r . mac) && over (& l . attrs , & r . attrs , eq_attr) } , _ => false , } }
};
}
