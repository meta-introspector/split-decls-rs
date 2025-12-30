// Generated macro for find_insert_calls (function)
macro_rules! Depcrate_entryfind_insert_calls {
() => {
// Module: crate::entry
// Provides: {"find_insert_calls"}
// Dependencies: {}
fn find_insert_calls < 'tcx > (cx : & LateContext < 'tcx > , contains_expr : & ContainsExpr < 'tcx > , expr : & 'tcx Expr < '_ > ,) -> Option < InsertSearchResults < 'tcx > > { let mut s = InsertSearcher { cx , map : contains_expr . map , key : contains_expr . key , ctxt : expr . span . ctxt () , spanless_eq : SpanlessEq :: new (cx) , allow_insert_closure : true , can_use_entry : true , in_tail_pos : true , is_single_insert : true , is_map_used : false , is_key_used : false , edits : Vec :: new () , loops : Vec :: new () , locals : HirIdSet :: default () , } ; s . visit_expr (expr) ; let allow_insert_closure = s . allow_insert_closure ; let is_single_insert = s . is_single_insert ; let is_key_used_and_no_copy = s . is_key_used && ! is_copy (cx , cx . typeck_results () . expr_ty (contains_expr . key)) ; let edits = s . edits ; s . can_use_entry . then_some (InsertSearchResults { edits , allow_insert_closure , is_single_insert , is_key_used_and_no_copy , }) }
};
}
