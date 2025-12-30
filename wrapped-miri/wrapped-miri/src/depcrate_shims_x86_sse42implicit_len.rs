// Generated macro for implicit_len (function)
macro_rules! Depcrate_shims_x86_sse42implicit_len {
() => {
// Module: crate::shims::x86::sse42
// Provides: {"implicit_len"}
// Dependencies: {}
# [doc = " Calculate the c-style string length for a given string `str`."] # [doc = " The string is either a length 16 array of bytes a length 8 array of two-byte words."] fn implicit_len < 'tcx > (ecx : & mut MiriInterpCx < 'tcx > , str : & OpTy < 'tcx > , imm : u8 ,) -> InterpResult < 'tcx , Option < u64 > > { let mut result = None ; let zero = ImmTy :: from_int (0 , str . layout . field (ecx , 0)) ; for i in 0 .. default_len :: < u64 > (imm) { let ch = ecx . read_immediate (& ecx . project_index (str , i) ?) ? ; let is_zero = ecx . binary_op (mir :: BinOp :: Eq , & ch , & zero) ? ; if is_zero . to_scalar () . to_bool () ? { result = Some (i) ; break ; } } interp_ok (result) }
};
}
