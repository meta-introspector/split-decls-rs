// Generated macro for abs_f128 (function)
macro_rules! Depcrate_codegen_f16_f128abs_f128 {
() => {
// Module: crate::codegen_f16_f128
// Provides: {"abs_f128"}
// Dependencies: {}
pub (crate) fn abs_f128 (fx : & mut FunctionCx < '_ , '_ , '_ > , value : Value) -> Value { let bits = fx . bcx . ins () . bitcast (types :: I128 , MemFlags :: new () , value) ; let (low , high) = fx . bcx . ins () . isplit (bits) ; let high = fx . bcx . ins () . band_imm (high , 0x7fff_ffff_ffff_ffff_u64 as i64) ; let bits = fx . bcx . ins () . iconcat (low , high) ; fx . bcx . ins () . bitcast (types :: F128 , MemFlags :: new () , bits) }
};
}
