// Generated macro for neg_f128 (function)
macro_rules! Depcrate_codegen_f16_f128neg_f128 {
() => {
// Module: crate::codegen_f16_f128
// Provides: {"neg_f128"}
// Dependencies: {}
pub (crate) fn neg_f128 (fx : & mut FunctionCx < '_ , '_ , '_ > , value : Value) -> Value { let bits = fx . bcx . ins () . bitcast (types :: I128 , MemFlags :: new () , value) ; let (low , high) = fx . bcx . ins () . isplit (bits) ; let high = fx . bcx . ins () . bxor_imm (high , 0x8000_0000_0000_0000_u64 as i64) ; let bits = fx . bcx . ins () . iconcat (low , high) ; fx . bcx . ins () . bitcast (types :: F128 , MemFlags :: new () , bits) }
};
}
