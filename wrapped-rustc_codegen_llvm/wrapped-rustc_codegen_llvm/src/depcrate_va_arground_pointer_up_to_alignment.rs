// Generated macro for round_pointer_up_to_alignment (function)
macro_rules! Depcrate_va_arground_pointer_up_to_alignment {
() => {
// Module: crate::va_arg
// Provides: {"round_pointer_up_to_alignment"}
// Dependencies: {}
fn round_pointer_up_to_alignment < 'll > (bx : & mut Builder < '_ , 'll , '_ > , addr : & 'll Value , align : Align , ptr_ty : & 'll Type ,) -> & 'll Value { let ptr = bx . inbounds_ptradd (addr , bx . const_i32 (align . bytes () as i32 - 1)) ; bx . call_intrinsic ("llvm.ptrmask" , & [ptr_ty , bx . type_i32 ()] , & [ptr , bx . const_int (bx . isize_ty , - (align . bytes () as isize) as i64)] ,) }
};
}
