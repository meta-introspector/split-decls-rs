// Generated macro for is_expr_same_field (function)
macro_rules! Depcrate_loops_while_let_on_iteratoris_expr_same_field {
() => {
// Module: crate::loops::while_let_on_iterator
// Provides: {"is_expr_same_field"}
// Dependencies: {}
fn is_expr_same_field (cx : & LateContext < '_ > , mut e : & Expr < '_ > , mut fields : & [Symbol] , path_res : Res) -> bool { loop { match (& e . kind , fields) { (& ExprKind :: Field (base , name) , [head_field , tail_fields @ ..]) if name . name == * head_field => { e = base ; fields = tail_fields ; } , (ExprKind :: Path (path) , []) => { break cx . qpath_res (path , e . hir_id) == path_res ; } , (& (ExprKind :: DropTemps (base) | ExprKind :: AddrOf (_ , _ , base) | ExprKind :: Type (base , _)) , _) => e = base , _ => break false , } } }
};
}
