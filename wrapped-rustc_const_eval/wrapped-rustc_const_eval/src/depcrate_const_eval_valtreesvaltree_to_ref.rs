// Generated macro for valtree_to_ref (function)
macro_rules! Depcrate_const_eval_valtreesvaltree_to_ref {
() => {
// Module: crate::const_eval::valtrees
// Provides: {"valtree_to_ref"}
// Dependencies: {}
# [doc = " Put a valtree into memory and return a reference to that."] fn valtree_to_ref < 'tcx > (ecx : & mut CompileTimeInterpCx < 'tcx > , valtree : ty :: ValTree < 'tcx > , pointee_ty : Ty < 'tcx > ,) -> Immediate { let pointee_place = create_valtree_place (ecx , ecx . layout_of (pointee_ty) . unwrap () , valtree) ; debug ! (? pointee_place) ; valtree_into_mplace (ecx , & pointee_place , valtree) ; dump_place (ecx , & pointee_place) ; intern_const_alloc_recursive (ecx , InternKind :: Constant , & pointee_place) . unwrap () ; pointee_place . to_ref (& ecx . tcx) }
};
}
