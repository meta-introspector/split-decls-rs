// Generated macro for neg_f16 (function)
macro_rules! Depcrate_codegen_f16_f128neg_f16 {
() => {
// Module: crate::codegen_f16_f128
// Provides: {"neg_f16"}
// Dependencies: {}
pub (crate) fn neg_f16 (fx : & mut FunctionCx < '_ , '_ , '_ > , value : Value) -> Value { let bits = fx . bcx . ins () . bitcast (types :: I16 , MemFlags :: new () , value) ; let bits = fx . bcx . ins () . bxor_imm (bits , 0x8000) ; fx . bcx . ins () . bitcast (types :: F16 , MemFlags :: new () , bits) }
};
}
