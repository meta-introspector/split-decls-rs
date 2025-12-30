// Generated macro for can_partially_move_ty (function)
macro_rules! Depcrate_tycan_partially_move_ty {
() => {
// Module: crate::ty
// Provides: {"can_partially_move_ty"}
// Dependencies: {}
# [doc = " Checks whether a type can be partially moved."] pub fn can_partially_move_ty < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { if has_drop (cx , ty) || is_copy (cx , ty) { return false ; } match ty . kind () { ty :: Param (_) => false , ty :: Adt (def , subs) => def . all_fields () . any (| f | ! is_copy (cx , f . ty (cx . tcx , subs))) , _ => true , } }
};
}
