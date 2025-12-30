// Generated macro for codegen_compare_bin_op (function)
macro_rules! Depcrate_numcodegen_compare_bin_op {
() => {
// Module: crate::num
// Provides: {"codegen_compare_bin_op"}
// Dependencies: {}
fn codegen_compare_bin_op < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , bin_op : BinOp , signed : bool , lhs : Value , rhs : Value ,) -> CValue < 'tcx > { let intcc = crate :: num :: bin_op_to_intcc (bin_op , signed) ; let val = fx . bcx . ins () . icmp (intcc , lhs , rhs) ; CValue :: by_val (val , fx . layout_of (fx . tcx . types . bool)) }
};
}
