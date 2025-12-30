// Generated macro for is_empty_slice (function)
macro_rules! Depcrate_manual_option_as_sliceis_empty_slice {
() => {
// Module: crate::manual_option_as_slice
// Provides: {"is_empty_slice"}
// Dependencies: {}
# [doc = " Returns if expr returns an empty slice. If:"] # [doc = " - An indexing operation to an empty array with a built-in range. `[][..]`"] # [doc = " - An indexing operation with a zero-ended range. `expr[..0]`"] # [doc = " - A reference to an empty array. `&[]`"] # [doc = " - Or a call to `Default::default`."] fn is_empty_slice (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { let expr = peel_hir_expr_refs (expr . peel_blocks ()) . 0 ; match expr . kind { ExprKind :: Index (arr , range , _) => match arr . kind { ExprKind :: Array ([]) => is_range_literal (range) , ExprKind :: Array (_) => { let Some (range) = clippy_utils :: higher :: Range :: hir (cx , range) else { return false ; } ; range . end . is_some_and (| e | clippy_utils :: is_integer_const (cx , e , 0)) } , _ => false , } , ExprKind :: Array ([]) => true , ExprKind :: Call (def , []) => def . res (cx) . is_diag_item (cx , sym :: default_fn) , _ => false , } }
};
}
