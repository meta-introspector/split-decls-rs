// Generated macro for apply_random_float_error_to_imm (function)
macro_rules! Depcrate_mathapply_random_float_error_to_imm {
() => {
// Module: crate::math
// Provides: {"apply_random_float_error_to_imm"}
// Dependencies: {}
# [doc = " Applies an error of `[-N, +N]` ULP to the given value."] # [doc = " Will fail if `val` is not a floating point number."] pub (crate) fn apply_random_float_error_to_imm < 'tcx > (ecx : & mut MiriInterpCx < 'tcx > , val : ImmTy < 'tcx > , max_error : u32 ,) -> InterpResult < 'tcx , ImmTy < 'tcx > > { let scalar = val . to_scalar_int () ? ; let res : ScalarInt = match val . layout . ty . kind () { ty :: Float (FloatTy :: F16) => apply_random_float_error_ulp (ecx , scalar . to_f16 () , max_error) . into () , ty :: Float (FloatTy :: F32) => apply_random_float_error_ulp (ecx , scalar . to_f32 () , max_error) . into () , ty :: Float (FloatTy :: F64) => apply_random_float_error_ulp (ecx , scalar . to_f64 () , max_error) . into () , ty :: Float (FloatTy :: F128) => apply_random_float_error_ulp (ecx , scalar . to_f128 () , max_error) . into () , _ => bug ! ("intrinsic called with non-float input type") , } ; interp_ok (ImmTy :: from_scalar_int (res , val . layout)) }
};
}
