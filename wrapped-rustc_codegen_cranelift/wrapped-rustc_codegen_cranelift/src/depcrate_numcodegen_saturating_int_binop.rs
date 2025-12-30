// Generated macro for codegen_saturating_int_binop (function)
macro_rules! Depcrate_numcodegen_saturating_int_binop {
() => {
// Module: crate::num
// Provides: {"codegen_saturating_int_binop"}
// Dependencies: {}
pub (crate) fn codegen_saturating_int_binop < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , bin_op : BinOp , lhs : CValue < 'tcx > , rhs : CValue < 'tcx > ,) -> CValue < 'tcx > { assert_eq ! (lhs . layout () . ty , rhs . layout () . ty) ; let signed = type_sign (lhs . layout () . ty) ; let clif_ty = fx . clif_type (lhs . layout () . ty) . unwrap () ; let (min , max) = type_min_max_value (& mut fx . bcx , clif_ty , signed) ; let checked_res = crate :: num :: codegen_checked_int_binop (fx , bin_op , lhs , rhs) ; let (val , has_overflow) = checked_res . load_scalar_pair (fx) ; let val = match (bin_op , signed) { (BinOp :: Add , false) => fx . bcx . ins () . select (has_overflow , max , val) , (BinOp :: Sub , false) => fx . bcx . ins () . select (has_overflow , min , val) , (BinOp :: Add , true) => { let rhs = rhs . load_scalar (fx) ; let rhs_ge_zero = fx . bcx . ins () . icmp_imm (IntCC :: SignedGreaterThanOrEqual , rhs , 0) ; let sat_val = fx . bcx . ins () . select (rhs_ge_zero , max , min) ; fx . bcx . ins () . select (has_overflow , sat_val , val) } (BinOp :: Sub , true) => { let rhs = rhs . load_scalar (fx) ; let rhs_ge_zero = fx . bcx . ins () . icmp_imm (IntCC :: SignedGreaterThanOrEqual , rhs , 0) ; let sat_val = fx . bcx . ins () . select (rhs_ge_zero , min , max) ; fx . bcx . ins () . select (has_overflow , sat_val , val) } _ => unreachable ! () , } ; CValue :: by_val (val , lhs . layout ()) }
};
}
