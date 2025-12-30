// Generated macro for fcmp (function)
macro_rules! Depcrate_codegen_f16_f128fcmp {
() => {
// Module: crate::codegen_f16_f128
// Provides: {"fcmp"}
// Dependencies: {}
pub (crate) fn fcmp (fx : & mut FunctionCx < '_ , '_ , '_ > , cc : FloatCC , lhs : Value , rhs : Value) -> Value { let ty = fx . bcx . func . dfg . value_type (lhs) ; match ty { types :: F32 | types :: F64 => fx . bcx . ins () . fcmp (cc , lhs , rhs) , types :: F16 => { let lhs = f16_to_f32 (fx , lhs) ; let rhs = f16_to_f32 (fx , rhs) ; fx . bcx . ins () . fcmp (cc , lhs , rhs) } types :: F128 => { let (name , int_cc) = match cc { FloatCC :: Equal => ("__eqtf2" , IntCC :: Equal) , FloatCC :: NotEqual => ("__netf2" , IntCC :: NotEqual) , FloatCC :: LessThan => ("__lttf2" , IntCC :: SignedLessThan) , FloatCC :: LessThanOrEqual => ("__letf2" , IntCC :: SignedLessThanOrEqual) , FloatCC :: GreaterThan => ("__gttf2" , IntCC :: SignedGreaterThan) , FloatCC :: GreaterThanOrEqual => ("__getf2" , IntCC :: SignedGreaterThanOrEqual) , _ => unreachable ! ("not currently used in rustc_codegen_cranelift: {cc:?}") , } ; let res = fx . lib_call (name , vec ! [AbiParam :: new (types :: F128) , AbiParam :: new (types :: F128)] , vec ! [AbiParam :: new (types :: I32)] , & [lhs , rhs] ,) [0] ; let zero = fx . bcx . ins () . iconst (types :: I32 , 0) ; let res = fx . bcx . ins () . icmp (int_cc , res , zero) ; res } _ => unreachable ! ("{ty:?}") , } }
};
}
