// Generated macro for round_first (function)
macro_rules! Depcrate_shims_x86round_first {
() => {
// Module: crate::shims::x86
// Provides: {"round_first"}
// Dependencies: {}
fn round_first < 'tcx , F : rustc_apfloat :: Float > (ecx : & mut crate :: MiriInterpCx < 'tcx > , left : & OpTy < 'tcx > , right : & OpTy < 'tcx > , rounding : & OpTy < 'tcx > , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , () > { let (left , left_len) = ecx . project_to_simd (left) ? ; let (right , right_len) = ecx . project_to_simd (right) ? ; let (dest , dest_len) = ecx . project_to_simd (dest) ? ; assert_eq ! (dest_len , left_len) ; assert_eq ! (dest_len , right_len) ; let rounding = rounding_from_imm (ecx . read_scalar (rounding) ? . to_i32 () ?) ? ; let op0 : F = ecx . read_scalar (& ecx . project_index (& right , 0) ?) ? . to_float () ? ; let res = op0 . round_to_integral (rounding) . value ; ecx . write_scalar (Scalar :: from_uint (res . to_bits () , Size :: from_bits (F :: BITS)) , & ecx . project_index (& dest , 0) ? ,) ? ; for i in 1 .. dest_len { ecx . copy_op (& ecx . project_index (& left , i) ? , & ecx . project_index (& dest , i) ?) ? ; } interp_ok (()) }
};
}
