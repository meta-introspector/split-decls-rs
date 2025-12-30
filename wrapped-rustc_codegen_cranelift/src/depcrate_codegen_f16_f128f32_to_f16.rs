// Generated macro for f32_to_f16 (function)
macro_rules! Depcrate_codegen_f16_f128f32_to_f16 {
() => {
// Module: crate::codegen_f16_f128
// Provides: {"f32_to_f16"}
// Dependencies: {}
pub (crate) fn f32_to_f16 (fx : & mut FunctionCx < '_ , '_ , '_ > , value : Value) -> Value { let ret_ty = if fx . tcx . sess . target . vendor == "apple" && fx . tcx . sess . target . arch == "x86_64" { types :: I16 } else { types :: F16 } ; let ret = fx . lib_call ("__truncsfhf2" , vec ! [AbiParam :: new (types :: F32)] , vec ! [AbiParam :: new (ret_ty)] , & [value] ,) [0] ; if ret_ty == types :: I16 { fx . bcx . ins () . bitcast (types :: F16 , MemFlags :: new () , ret) } else { ret } }
};
}
