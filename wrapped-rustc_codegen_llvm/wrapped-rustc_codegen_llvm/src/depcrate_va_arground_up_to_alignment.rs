// Generated macro for round_up_to_alignment (function)
macro_rules! Depcrate_va_arground_up_to_alignment {
() => {
// Module: crate::va_arg
// Provides: {"round_up_to_alignment"}
// Dependencies: {}
fn round_up_to_alignment < 'll > (bx : & mut Builder < '_ , 'll , '_ > , mut value : & 'll Value , align : Align ,) -> & 'll Value { value = bx . add (value , bx . cx () . const_i32 (align . bytes () as i32 - 1)) ; return bx . and (value , bx . cx () . const_i32 (- (align . bytes () as i32))) ; }
};
}
