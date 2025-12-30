// Generated macro for for_each_local_assignment (function)
macro_rules! Depcrate_visitorsfor_each_local_assignment {
() => {
// Module: crate::visitors
// Provides: {"for_each_local_assignment"}
// Dependencies: {}
# [doc = " Runs the given function for each path expression referencing the given local which occur after"] # [doc = " the given expression."] pub fn for_each_local_assignment < 'tcx , B > (cx : & LateContext < 'tcx > , local_id : HirId , f : impl FnMut (& 'tcx Expr < 'tcx >) -> ControlFlow < B > ,) -> ControlFlow < B > { struct V < 'cx , 'tcx , F , B > { cx : & 'cx LateContext < 'tcx > , local_id : HirId , res : ControlFlow < B > , f : F , } impl < 'tcx , F : FnMut (& 'tcx Expr < 'tcx >) -> ControlFlow < B > , B > Visitor < 'tcx > for V < '_ , 'tcx , F , B > { type NestedFilter = nested_filter :: OnlyBodies ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } fn visit_expr (& mut self , e : & 'tcx Expr < 'tcx >) { if let ExprKind :: Assign (lhs , rhs , _) = e . kind && self . res . is_continue () && lhs . res_local_id () == Some (self . local_id) { self . res = (self . f) (rhs) ; self . visit_expr (rhs) ; } else { walk_expr (self , e) ; } } } if let Some (b) = get_enclosing_block (cx , local_id) { let mut v = V { cx , local_id , res : ControlFlow :: Continue (()) , f , } ; v . visit_block (b) ; v . res } else { ControlFlow :: Continue (()) } }
};
}
