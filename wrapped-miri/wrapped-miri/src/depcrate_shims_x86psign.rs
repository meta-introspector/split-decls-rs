// Generated macro for psign (function)
macro_rules! Depcrate_shims_x86psign {
() => {
// Module: crate::shims::x86
// Provides: {"psign"}
// Dependencies: {}
# [doc = " Negates elements from `left` when the corresponding element in"] # [doc = " `right` is negative. If an element from `right` is zero, zero"] # [doc = " is written to the corresponding output element."] # [doc = " In other words, multiplies `left` with `right.signum()`."] fn psign < 'tcx > (ecx : & mut crate :: MiriInterpCx < 'tcx > , left : & OpTy < 'tcx > , right : & OpTy < 'tcx > , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , () > { let (left , left_len) = ecx . project_to_simd (left) ? ; let (right , right_len) = ecx . project_to_simd (right) ? ; let (dest , dest_len) = ecx . project_to_simd (dest) ? ; assert_eq ! (dest_len , left_len) ; assert_eq ! (dest_len , right_len) ; for i in 0 .. dest_len { let dest = ecx . project_index (& dest , i) ? ; let left = ecx . read_immediate (& ecx . project_index (& left , i) ?) ? ; let right = ecx . read_scalar (& ecx . project_index (& right , i) ?) ? . to_int (dest . layout . size) ? ; let res = ecx . binary_op (mir :: BinOp :: Mul , & left , & ImmTy :: from_int (right . signum () , dest . layout)) ? ; ecx . write_immediate (* res , & dest) ? ; } interp_ok (()) }
};
}
