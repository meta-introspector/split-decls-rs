// Generated macro for codegen_float_max (function)
macro_rules! Depcrate_numcodegen_float_max {
() => {
// Module: crate::num
// Provides: {"codegen_float_max"}
// Dependencies: {}
pub (crate) fn codegen_float_max (fx : & mut FunctionCx < '_ , '_ , '_ > , a : Value , b : Value) -> Value { let a_is_nan = codegen_f16_f128 :: fcmp (fx , FloatCC :: NotEqual , a , a) ; let a_le_b = codegen_f16_f128 :: fcmp (fx , FloatCC :: LessThanOrEqual , a , b) ; let temp = fx . bcx . ins () . select (a_le_b , b , a) ; fx . bcx . ins () . select (a_is_nan , b , temp) }
};
}
