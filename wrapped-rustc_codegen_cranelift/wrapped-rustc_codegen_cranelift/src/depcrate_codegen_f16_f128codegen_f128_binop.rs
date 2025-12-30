// Generated macro for codegen_f128_binop (function)
macro_rules! Depcrate_codegen_f16_f128codegen_f128_binop {
() => {
// Module: crate::codegen_f16_f128
// Provides: {"codegen_f128_binop"}
// Dependencies: {}
pub (crate) fn codegen_f128_binop (fx : & mut FunctionCx < '_ , '_ , '_ > , bin_op : BinOp , lhs : Value , rhs : Value ,) -> Value { let name = match bin_op { BinOp :: Add => "__addtf3" , BinOp :: Sub => "__subtf3" , BinOp :: Mul => "__multf3" , BinOp :: Div => "__divtf3" , _ => unreachable ! ("handled in `codegen_float_binop`") , } ; fx . lib_call (name , vec ! [AbiParam :: new (types :: F128) , AbiParam :: new (types :: F128)] , vec ! [AbiParam :: new (types :: F128)] , & [lhs , rhs] ,) [0] }
};
}
