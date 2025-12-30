// Generated macro for convert_float_to_int (function)
macro_rules! Depcrate_shims_x86convert_float_to_int {
() => {
// Module: crate::shims::x86
// Provides: {"convert_float_to_int"}
// Dependencies: {}
# [doc = " Converts each element of `op` from floating point to signed integer."] # [doc = ""] # [doc = " When the input value is NaN or out of range, fall back to minimum value."] # [doc = ""] # [doc = " If `op` has more elements than `dest`, extra elements are ignored. If `op`"] # [doc = " has less elements than `dest`, the rest is filled with zeros."] fn convert_float_to_int < 'tcx > (ecx : & mut crate :: MiriInterpCx < 'tcx > , op : & OpTy < 'tcx > , rnd : rustc_apfloat :: Round , dest : & MPlaceTy < 'tcx > ,) -> InterpResult < 'tcx , () > { let (op , op_len) = ecx . project_to_simd (op) ? ; let (dest , dest_len) = ecx . project_to_simd (dest) ? ; assert ! (matches ! (dest . layout . field (ecx , 0) . ty . kind () , ty :: Int (_))) ; for i in 0 .. op_len . min (dest_len) { let op = ecx . read_immediate (& ecx . project_index (& op , i) ?) ? ; let dest = ecx . project_index (& dest , i) ? ; let res = ecx . float_to_int_checked (& op , dest . layout , rnd) ? . unwrap_or_else (| | { ImmTy :: from_int (dest . layout . size . signed_int_min () , dest . layout) }) ; ecx . write_immediate (* res , & dest) ? ; } for i in op_len .. dest_len { let dest = ecx . project_index (& dest , i) ? ; ecx . write_scalar (Scalar :: from_int (0 , dest . layout . size) , & dest) ? ; } interp_ok (()) }
};
}
