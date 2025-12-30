// Generated macro for bin_op_float (function)
macro_rules! Depcrate_shims_x86bin_op_float {
() => {
// Module: crate::shims::x86
// Provides: {"bin_op_float"}
// Dependencies: {}
# [doc = " Performs `which` scalar operation on `left` and `right` and returns"] # [doc = " the result."] fn bin_op_float < 'tcx , F : rustc_apfloat :: Float > (which : FloatBinOp , left : & ImmTy < 'tcx > , right : & ImmTy < 'tcx > ,) -> InterpResult < 'tcx , Scalar > { match which { FloatBinOp :: Cmp { gt , lt , eq , unord } => { let left = left . to_scalar () . to_float :: < F > () ? ; let right = right . to_scalar () . to_float :: < F > () ? ; let res = match left . partial_cmp (& right) { None => unord , Some (std :: cmp :: Ordering :: Less) => lt , Some (std :: cmp :: Ordering :: Equal) => eq , Some (std :: cmp :: Ordering :: Greater) => gt , } ; interp_ok (bool_to_simd_element (res , Size :: from_bits (F :: BITS))) } FloatBinOp :: Min => { let left_scalar = left . to_scalar () ; let left = left_scalar . to_float :: < F > () ? ; let right_scalar = right . to_scalar () ; let right = right_scalar . to_float :: < F > () ? ; if (left == F :: ZERO && right == F :: ZERO) || left . is_nan () || right . is_nan () || left >= right { interp_ok (right_scalar) } else { interp_ok (left_scalar) } } FloatBinOp :: Max => { let left_scalar = left . to_scalar () ; let left = left_scalar . to_float :: < F > () ? ; let right_scalar = right . to_scalar () ; let right = right_scalar . to_float :: < F > () ? ; if (left == F :: ZERO && right == F :: ZERO) || left . is_nan () || right . is_nan () || left <= right { interp_ok (right_scalar) } else { interp_ok (left_scalar) } } } }
};
}
