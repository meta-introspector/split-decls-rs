// Generated macro for skip_fields_and_path (function)
macro_rules! Depcrate_loops_while_let_on_iteratorskip_fields_and_path {
() => {
// Module: crate::loops::while_let_on_iterator
// Provides: {"skip_fields_and_path"}
// Dependencies: {}
# [doc = " Strips off all field and path expressions. This will return true if a field or path has been"] # [doc = " skipped. Used to skip them after failing to check for equality."] fn skip_fields_and_path < 'tcx > (expr : & 'tcx Expr < '_ >) -> (Option < & 'tcx Expr < 'tcx > > , bool) { let mut e = expr ; let e = loop { match e . kind { ExprKind :: Field (base , _) | ExprKind :: DropTemps (base) | ExprKind :: Type (base , _) => e = base , ExprKind :: Path (_) => return (None , true) , _ => break e , } } ; (Some (e) , e . hir_id != expr . hir_id) }
};
}
