// Generated macro for codegen_bool_binop (function)
macro_rules! Depcrate_numcodegen_bool_binop {
() => {
// Module: crate::num
// Provides: {"codegen_bool_binop"}
// Dependencies: {}
fn codegen_bool_binop < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , bin_op : BinOp , in_lhs : CValue < 'tcx > , in_rhs : CValue < 'tcx > ,) -> CValue < 'tcx > { let lhs = in_lhs . load_scalar (fx) ; let rhs = in_rhs . load_scalar (fx) ; let b = fx . bcx . ins () ; let res = match bin_op { BinOp :: BitXor => b . bxor (lhs , rhs) , BinOp :: BitAnd => b . band (lhs , rhs) , BinOp :: BitOr => b . bor (lhs , rhs) , _ => unreachable ! ("{:?}({:?}, {:?})" , bin_op , in_lhs , in_rhs) , } ; CValue :: by_val (res , fx . layout_of (fx . tcx . types . bool)) }
};
}
