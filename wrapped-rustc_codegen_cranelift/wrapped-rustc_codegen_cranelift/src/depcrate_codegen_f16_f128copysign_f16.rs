// Generated macro for copysign_f16 (function)
macro_rules! Depcrate_codegen_f16_f128copysign_f16 {
() => {
// Module: crate::codegen_f16_f128
// Provides: {"copysign_f16"}
// Dependencies: {}
pub (crate) fn copysign_f16 (fx : & mut FunctionCx < '_ , '_ , '_ > , lhs : Value , rhs : Value) -> Value { let lhs = fx . bcx . ins () . bitcast (types :: I16 , MemFlags :: new () , lhs) ; let rhs = fx . bcx . ins () . bitcast (types :: I16 , MemFlags :: new () , rhs) ; let res = fx . bcx . ins () . band_imm (lhs , 0x7fff) ; let sign = fx . bcx . ins () . band_imm (rhs , 0x8000) ; let res = fx . bcx . ins () . bor (res , sign) ; fx . bcx . ins () . bitcast (types :: F16 , MemFlags :: new () , res) }
};
}
