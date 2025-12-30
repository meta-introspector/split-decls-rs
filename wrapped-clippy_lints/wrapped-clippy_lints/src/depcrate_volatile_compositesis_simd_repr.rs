// Generated macro for is_simd_repr (function)
macro_rules! Depcrate_volatile_compositesis_simd_repr {
() => {
// Module: crate::volatile_composites
// Provides: {"is_simd_repr"}
// Dependencies: {}
# [doc = " SIMD can be useful to get larger single loads/stores, though this is still"] # [doc = " pretty machine-dependent."] fn is_simd_repr < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { if let ty :: Adt (adt_def , _args) = ty . kind () && adt_def . is_struct () && adt_def . repr () . simd () { let (_size , simdty) = ty . simd_size_and_type (cx . tcx) ; is_volatile_safe_ty (cx , simdty) } else { false } }
};
}
