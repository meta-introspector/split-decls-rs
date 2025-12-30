// Generated macro for is_cast_from_ref_to_mut_ptr (function)
macro_rules! Depcrate_reference_castingis_cast_from_ref_to_mut_ptr {
() => {
// Module: crate::reference_casting
// Provides: {"is_cast_from_ref_to_mut_ptr"}
// Dependencies: {}
fn is_cast_from_ref_to_mut_ptr < 'tcx > (cx : & LateContext < 'tcx > , orig_expr : & 'tcx Expr < 'tcx > , mut peel_casts : impl FnMut () -> (& 'tcx Expr < 'tcx > , bool) ,) -> Option < bool > { let end_ty = cx . typeck_results () . node_type (orig_expr . hir_id) ; if ! matches ! (end_ty . kind () , ty :: RawPtr (_ , Mutability :: Mut)) { return None ; } let (e , need_check_freeze) = peel_casts () ; let start_ty = cx . typeck_results () . node_type (e . hir_id) ; if let ty :: Ref (_ , inner_ty , Mutability :: Not) = start_ty . kind () { let inner_ty_has_interior_mutability = ! inner_ty . is_freeze (cx . tcx , cx . typing_env ()) && inner_ty . has_concrete_skeleton () ; (! need_check_freeze || ! inner_ty_has_interior_mutability) . then_some (inner_ty_has_interior_mutability) } else { None } }
};
}
