// Generated macro for fmax_f128 (function)
macro_rules! Depcrate_codegen_f16_f128fmax_f128 {
() => {
// Module: crate::codegen_f16_f128
// Provides: {"fmax_f128"}
// Dependencies: {}
pub (crate) fn fmax_f128 (fx : & mut FunctionCx < '_ , '_ , '_ > , a : Value , b : Value) -> Value { fx . lib_call ("fmaximumf128" , vec ! [AbiParam :: new (types :: F128) , AbiParam :: new (types :: F128)] , vec ! [AbiParam :: new (types :: F128)] , & [a , b] ,) [0] }
};
}
