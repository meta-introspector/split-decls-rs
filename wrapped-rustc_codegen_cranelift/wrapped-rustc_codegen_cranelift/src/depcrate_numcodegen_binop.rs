// Generated macro for codegen_binop (function)
macro_rules! Depcrate_numcodegen_binop {
() => {
// Module: crate::num
// Provides: {"codegen_binop"}
// Dependencies: {}
pub (crate) fn codegen_binop < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , bin_op : BinOp , in_lhs : CValue < 'tcx > , in_rhs : CValue < 'tcx > ,) -> CValue < 'tcx > { match bin_op { BinOp :: Eq | BinOp :: Lt | BinOp :: Le | BinOp :: Ne | BinOp :: Ge | BinOp :: Gt => { match in_lhs . layout () . ty . kind () { ty :: Bool | ty :: Uint (_) | ty :: Int (_) | ty :: Char => { let signed = type_sign (in_lhs . layout () . ty) ; let lhs = in_lhs . load_scalar (fx) ; let rhs = in_rhs . load_scalar (fx) ; return codegen_compare_bin_op (fx , bin_op , signed , lhs , rhs) ; } _ => { } } } BinOp :: Cmp => match in_lhs . layout () . ty . kind () { ty :: Bool | ty :: Uint (_) | ty :: Int (_) | ty :: Char => { let signed = type_sign (in_lhs . layout () . ty) ; let lhs = in_lhs . load_scalar (fx) ; let rhs = in_rhs . load_scalar (fx) ; return codegen_three_way_compare (fx , signed , lhs , rhs) ; } _ => { } } , _ => { } } match in_lhs . layout () . ty . kind () { ty :: Bool => crate :: num :: codegen_bool_binop (fx , bin_op , in_lhs , in_rhs) , ty :: Uint (_) | ty :: Int (_) => crate :: num :: codegen_int_binop (fx , bin_op , in_lhs , in_rhs) , ty :: Float (_) => crate :: num :: codegen_float_binop (fx , bin_op , in_lhs , in_rhs) , ty :: RawPtr (..) | ty :: FnPtr (..) => crate :: num :: codegen_ptr_binop (fx , bin_op , in_lhs , in_rhs) , _ => unreachable ! ("{:?}({:?}, {:?})" , bin_op , in_lhs . layout () . ty , in_rhs . layout () . ty) , } }
};
}
