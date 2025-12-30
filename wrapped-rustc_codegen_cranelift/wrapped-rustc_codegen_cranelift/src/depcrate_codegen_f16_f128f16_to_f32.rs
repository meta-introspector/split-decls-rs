// Generated macro for f16_to_f32 (function)
macro_rules! Depcrate_codegen_f16_f128f16_to_f32 {
() => {
// Module: crate::codegen_f16_f128
// Provides: {"f16_to_f32"}
// Dependencies: {}
pub (crate) fn f16_to_f32 (fx : & mut FunctionCx < '_ , '_ , '_ > , value : Value) -> Value { let (value , arg_ty) = if fx . tcx . sess . target . vendor == "apple" && fx . tcx . sess . target . arch == "x86_64" { (fx . bcx . ins () . bitcast (types :: I16 , MemFlags :: new () , value) , lib_call_arg_param (fx . tcx , types :: I16 , false) ,) } else { (value , AbiParam :: new (types :: F16)) } ; fx . lib_call ("__extendhfsf2" , vec ! [arg_ty] , vec ! [AbiParam :: new (types :: F32)] , & [value]) [0] }
};
}
