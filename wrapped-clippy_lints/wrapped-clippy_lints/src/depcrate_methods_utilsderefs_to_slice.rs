// Generated macro for derefs_to_slice (function)
macro_rules! Depcrate_methods_utilsderefs_to_slice {
() => {
// Module: crate::methods::utils
// Provides: {"derefs_to_slice"}
// Dependencies: {}
# [doc = " Checks if `expr`, of type `ty`, corresponds to a slice or can be dereferenced to a slice, or if"] # [doc = " `expr` is a method call to `.iter()` on such a type. In these cases, return the slice-like"] # [doc = " expression."] pub (super) fn derefs_to_slice < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , ty : Ty < 'tcx > ,) -> Option < & 'tcx Expr < 'tcx > > { fn may_slice < 'a > (cx : & LateContext < 'a > , ty : Ty < 'a >) -> bool { match ty . kind () { ty :: Slice (_) => true , ty :: Adt (..) if let Some (boxed) = ty . boxed_ty () => may_slice (cx , boxed) , ty :: Adt (..) => ty . is_diag_item (cx , sym :: Vec) , ty :: Array (_ , size) => size . try_to_target_usize (cx . tcx) . is_some () , ty :: Ref (_ , inner , _) => may_slice (cx , * inner) , _ => false , } } if let ExprKind :: MethodCall (path , self_arg , ..) = & expr . kind { if path . ident . name == sym :: iter && may_slice (cx , cx . typeck_results () . expr_ty (self_arg)) { Some (self_arg) } else { None } } else { match ty . kind () { ty :: Slice (_) => Some (expr) , _ if ty . boxed_ty () . is_some_and (| boxed | may_slice (cx , boxed)) => Some (expr) , ty :: Ref (_ , inner , _) => { if may_slice (cx , * inner) { Some (expr) } else { None } } , _ => None , } } }
};
}
