// Generated macro for uses_iter (function)
macro_rules! Depcrate_loops_while_let_on_iteratoruses_iter {
() => {
// Module: crate::loops::while_let_on_iterator
// Provides: {"uses_iter"}
// Dependencies: {}
# [doc = " Checks if the given expression uses the iterator."] fn uses_iter < 'tcx > (cx : & LateContext < 'tcx > , iter_expr : & IterExpr , container : & 'tcx Expr < '_ >) -> bool { struct V < 'a , 'b , 'tcx > { cx : & 'a LateContext < 'tcx > , iter_expr : & 'b IterExpr , } impl < 'tcx > Visitor < 'tcx > for V < '_ , '_ , 'tcx > { type Result = ControlFlow < () > ; fn visit_expr (& mut self , e : & 'tcx Expr < '_ >) -> Self :: Result { if is_expr_same_child_or_parent_field (self . cx , e , & self . iter_expr . fields , self . iter_expr . path) { ControlFlow :: Break (()) } else if let (e , true) = skip_fields_and_path (e) { if let Some (e) = e { self . visit_expr (e) } else { ControlFlow :: Continue (()) } } else if let ExprKind :: Closure (& Closure { body : id , .. }) = e . kind { if is_res_used (self . cx , self . iter_expr . path , id) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } } else { walk_expr (self , e) } } } let mut v = V { cx , iter_expr } ; v . visit_expr (container) . is_break () }
};
}
