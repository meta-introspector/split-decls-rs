// Generated macro for llvm_add_sub (function)
macro_rules! Depcrate_intrinsics_llvm_x86llvm_add_sub {
() => {
// Module: crate::intrinsics::llvm_x86
// Provides: {"llvm_add_sub"}
// Dependencies: {}
fn llvm_add_sub < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , bin_op : BinOp , cb_in : Value , a : CValue < 'tcx > , b : CValue < 'tcx > ,) -> (Value , Value) { assert_eq ! (a . layout () . ty , b . layout () . ty) ; let int0 = crate :: num :: codegen_checked_int_binop (fx , bin_op , a , b) ; let c = int0 . value_field (fx , FieldIdx :: ZERO) ; let cb0 = int0 . value_field (fx , FieldIdx :: new (1)) . load_scalar (fx) ; let clif_ty = fx . clif_type (a . layout () . ty) . unwrap () ; let cb_in_as_int = fx . bcx . ins () . uextend (clif_ty , cb_in) ; let cb_in_as_int = CValue :: by_val (cb_in_as_int , fx . layout_of (a . layout () . ty)) ; let int1 = crate :: num :: codegen_checked_int_binop (fx , bin_op , c , cb_in_as_int) ; let (c , cb1) = int1 . load_scalar_pair (fx) ; let cb_out = fx . bcx . ins () . bor (cb0 , cb1) ; (cb_out , c) }
};
}
