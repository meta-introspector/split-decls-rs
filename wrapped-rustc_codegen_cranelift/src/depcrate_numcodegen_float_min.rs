// Generated macro for codegen_float_min (function)
macro_rules! Depcrate_numcodegen_float_min {
() => {
// Module: crate::num
// Provides: {"codegen_float_min"}
// Dependencies: {}
pub (crate) fn codegen_float_min (fx : & mut FunctionCx < '_ , '_ , '_ > , a : Value , b : Value) -> Value { let a_is_nan = codegen_f16_f128 :: fcmp (fx , FloatCC :: NotEqual , a , a) ; let a_ge_b = codegen_f16_f128 :: fcmp (fx , FloatCC :: GreaterThanOrEqual , a , b) ; let temp = fx . bcx . ins () . select (a_ge_b , b , a) ; fx . bcx . ins () . select (a_is_nan , b , temp) }
};
}
