// Generated macro for local_base (function)
macro_rules! Depcrate_replace_boxlocal_base {
() => {
// Module: crate::replace_box
// Provides: {"local_base"}
// Dependencies: {}
# [doc = " If `expr` is a local variable with optional field accesses, return it."] fn local_base (expr : & Expr < '_ >) -> Option < IdFields > { match expr . kind { ExprKind :: Path (qpath) => qpath . res_local_id () . map (| id | (id , Vec :: new ())) , ExprKind :: Field (expr , field) => local_base (expr) . map (| (id , mut fields) | { fields . push (field . name) ; (id , fields) }) , _ => None , } }
};
}
