// Generated macro for check_range_bounds (function)
macro_rules! Depcrate_rangescheck_range_bounds {
() => {
// Module: crate::ranges
// Provides: {"check_range_bounds"}
// Dependencies: {}
fn check_range_bounds < 'a > (cx : & 'a LateContext < '_ > , ex : & 'a Expr < '_ >) -> Option < RangeBounds < 'a > > { if let ExprKind :: Binary (ref op , l , r) = ex . kind { let (inclusive , ordering) = match op . node { BinOpKind :: Gt => (false , Ordering :: Greater) , BinOpKind :: Ge => (true , Ordering :: Greater) , BinOpKind :: Lt => (false , Ordering :: Less) , BinOpKind :: Le => (true , Ordering :: Less) , _ => return None , } ; if let Some (id) = l . res_local_id () { if let Some (c) = ConstEvalCtxt :: new (cx) . eval (r) { return Some (RangeBounds { val : c , expr : r , id , name_span : l . span , val_span : r . span , ord : ordering , inc : inclusive , }) ; } } else if let Some (id) = r . res_local_id () && let Some (c) = ConstEvalCtxt :: new (cx) . eval (l) { return Some (RangeBounds { val : c , expr : l , id , name_span : r . span , val_span : l . span , ord : ordering . reverse () , inc : inclusive , }) ; } } None }
};
}
