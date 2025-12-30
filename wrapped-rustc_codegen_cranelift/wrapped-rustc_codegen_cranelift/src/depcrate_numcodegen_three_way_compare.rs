// Generated macro for codegen_three_way_compare (function)
macro_rules! Depcrate_numcodegen_three_way_compare {
() => {
// Module: crate::num
// Provides: {"codegen_three_way_compare"}
// Dependencies: {}
fn codegen_three_way_compare < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , signed : bool , lhs : Value , rhs : Value ,) -> CValue < 'tcx > { let gt_cc = crate :: num :: bin_op_to_intcc (BinOp :: Gt , signed) ; let lt_cc = crate :: num :: bin_op_to_intcc (BinOp :: Lt , signed) ; let gt = fx . bcx . ins () . icmp (gt_cc , lhs , rhs) ; let lt = fx . bcx . ins () . icmp (lt_cc , lhs , rhs) ; let val = fx . bcx . ins () . isub (gt , lt) ; CValue :: by_val (val , fx . layout_of (fx . tcx . ty_ordering_enum (fx . mir . span))) }
};
}
