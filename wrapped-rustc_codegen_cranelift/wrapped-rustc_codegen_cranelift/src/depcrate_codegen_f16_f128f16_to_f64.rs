// Generated macro for f16_to_f64 (function)
macro_rules! Depcrate_codegen_f16_f128f16_to_f64 {
() => {
// Module: crate::codegen_f16_f128
// Provides: {"f16_to_f64"}
// Dependencies: {}
fn f16_to_f64 (fx : & mut FunctionCx < '_ , '_ , '_ > , value : Value) -> Value { let ret = f16_to_f32 (fx , value) ; fx . bcx . ins () . fpromote (types :: F64 , ret) }
};
}
