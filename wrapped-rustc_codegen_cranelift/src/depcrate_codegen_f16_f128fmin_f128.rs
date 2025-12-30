// Generated macro for fmin_f128 (function)
macro_rules! Depcrate_codegen_f16_f128fmin_f128 {
() => {
// Module: crate::codegen_f16_f128
// Provides: {"fmin_f128"}
// Dependencies: {}
pub (crate) fn fmin_f128 (fx : & mut FunctionCx < '_ , '_ , '_ > , a : Value , b : Value) -> Value { fx . lib_call ("fminimumf128" , vec ! [AbiParam :: new (types :: F128) , AbiParam :: new (types :: F128)] , vec ! [AbiParam :: new (types :: F128)] , & [a , b] ,) [0] }
};
}
