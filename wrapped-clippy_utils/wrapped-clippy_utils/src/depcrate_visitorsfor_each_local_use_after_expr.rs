// Generated macro for for_each_local_use_after_expr (function)
macro_rules! Depcrate_visitorsfor_each_local_use_after_expr {
() => {
// Module: crate::visitors
// Provides: {"for_each_local_use_after_expr"}
// Dependencies: {}
# [doc = " Runs the given function for each path expression referencing the given local which occur after"] # [doc = " the given expression."] pub fn for_each_local_use_after_expr < 'tcx , B > (cx : & LateContext < 'tcx > , local_id : HirId , expr_id : HirId , f : impl FnMut (& 'tcx Expr < 'tcx >) -> ControlFlow < B > ,) -> ControlFlow < B > { struct V < 'cx , 'tcx , F , B > { cx : & 'cx LateContext < 'tcx > , local_id : HirId , expr_id : HirId , found : bool , res : ControlFlow < B > , f : F , } impl < 'tcx , F : FnMut (& 'tcx Expr < 'tcx >) -> ControlFlow < B > , B > Visitor < 'tcx > for V < '_ , 'tcx , F , B > { type NestedFilter = nested_filter :: OnlyBodies ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } fn visit_expr (& mut self , e : & 'tcx Expr < 'tcx >) { if ! self . found { if e . hir_id == self . expr_id { self . found = true ; } else { walk_expr (self , e) ; } return ; } if self . res . is_break () { return ; } if e . res_local_id () == Some (self . local_id) { self . res = (self . f) (e) ; } else { walk_expr (self , e) ; } } } if let Some (b) = get_enclosing_block (cx , local_id) { let mut v = V { cx , local_id , expr_id , found : false , res : ControlFlow :: Continue (()) , f , } ; v . visit_block (b) ; v . res } else { ControlFlow :: Continue (()) } }
};
}
