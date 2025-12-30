// Generated macro for impl_1942 (impl)
macro_rules! Depcrate_entryimpl_1942 {
() => {
// Module: crate::entry
// Provides: {"impl_1942"}
// Dependencies: {}
impl < 'tcx > InsertSearcher < '_ , 'tcx > { # [doc = " Visit the expression as a branch in control flow. Multiple insert calls can be used, but"] # [doc = " only if they are on separate code paths. This will return whether the map was used in the"] # [doc = " given expression."] fn visit_cond_arm (& mut self , e : & 'tcx Expr < '_ >) -> bool { let is_map_used = self . is_map_used ; let in_tail_pos = self . in_tail_pos ; self . visit_expr (e) ; let res = self . is_map_used ; self . is_map_used = is_map_used ; self . in_tail_pos = in_tail_pos ; res } # [doc = " Visit an expression which is not itself in a tail position, but other sibling expressions"] # [doc = " may be. e.g. if conditions"] fn visit_non_tail_expr (& mut self , e : & 'tcx Expr < '_ >) { let in_tail_pos = self . in_tail_pos ; self . in_tail_pos = false ; self . visit_expr (e) ; self . in_tail_pos = in_tail_pos ; } # [doc = " Visit the key and value expression of an insert expression."] # [doc = " There may not be uses of the map in either of those two either."] fn visit_insert_expr_arguments (& mut self , e : & InsertExpr < 'tcx >) { let in_tail_pos = self . in_tail_pos ; let allow_insert_closure = self . allow_insert_closure ; let is_single_insert = self . is_single_insert ; walk_expr (self , e . key) ; walk_expr (self , e . value) ; self . in_tail_pos = in_tail_pos ; self . allow_insert_closure = allow_insert_closure ; self . is_single_insert = is_single_insert ; } }
};
}
