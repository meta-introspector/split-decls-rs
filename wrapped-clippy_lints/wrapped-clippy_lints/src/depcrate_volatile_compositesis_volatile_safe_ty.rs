// Generated macro for is_volatile_safe_ty (function)
macro_rules! Depcrate_volatile_compositesis_volatile_safe_ty {
() => {
// Module: crate::volatile_composites
// Provides: {"is_volatile_safe_ty"}
// Dependencies: {}
# [doc = " Top-level predicate for whether a type is volatile-safe or not."] fn is_volatile_safe_ty < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { ty . is_primitive () || is_narrow_ptr (cx , ty) || is_zero_sized_ty (cx , ty) || is_enum_repr_c (cx , ty) || is_simd_repr (cx , ty) || is_struct_repr_transparent (cx , ty) || ty . has_non_region_param () }
};
}
