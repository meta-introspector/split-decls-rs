// Generated macro for copysign_f128 (function)
macro_rules! Depcrate_codegen_f16_f128copysign_f128 {
() => {
// Module: crate::codegen_f16_f128
// Provides: {"copysign_f128"}
// Dependencies: {}
pub (crate) fn copysign_f128 (fx : & mut FunctionCx < '_ , '_ , '_ > , lhs : Value , rhs : Value) -> Value { let lhs = fx . bcx . ins () . bitcast (types :: I128 , MemFlags :: new () , lhs) ; let rhs = fx . bcx . ins () . bitcast (types :: I128 , MemFlags :: new () , rhs) ; let (low , lhs_high) = fx . bcx . ins () . isplit (lhs) ; let (_ , rhs_high) = fx . bcx . ins () . isplit (rhs) ; let high = fx . bcx . ins () . band_imm (lhs_high , 0x7fff_ffff_ffff_ffff_u64 as i64) ; let sign = fx . bcx . ins () . band_imm (rhs_high , 0x8000_0000_0000_0000_u64 as i64) ; let high = fx . bcx . ins () . bor (high , sign) ; let res = fx . bcx . ins () . iconcat (low , high) ; fx . bcx . ins () . bitcast (types :: F128 , MemFlags :: new () , res) }
};
}
