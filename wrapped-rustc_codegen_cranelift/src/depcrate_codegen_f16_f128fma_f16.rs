// Generated macro for fma_f16 (function)
macro_rules! Depcrate_codegen_f16_f128fma_f16 {
() => {
// Module: crate::codegen_f16_f128
// Provides: {"fma_f16"}
// Dependencies: {}
pub (crate) fn fma_f16 (fx : & mut FunctionCx < '_ , '_ , '_ > , x : Value , y : Value , z : Value) -> Value { let x = f16_to_f64 (fx , x) ; let y = f16_to_f64 (fx , y) ; let z = f16_to_f64 (fx , z) ; let res = fx . bcx . ins () . fma (x , y , z) ; f64_to_f16 (fx , res) }
};
}
