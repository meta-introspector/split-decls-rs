// Generated macro for round_up_const_value_to_alignment (function)
macro_rules! Depcrate_mir_placeround_up_const_value_to_alignment {
() => {
// Module: crate::mir::place
// Provides: {"round_up_const_value_to_alignment"}
// Dependencies: {}
fn round_up_const_value_to_alignment < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , value : Bx :: Value , align : Bx :: Value ,) -> Bx :: Value { let one = bx . const_usize (1) ; let align_minus_1 = bx . sub (align , one) ; let neg_value = bx . neg (value) ; let offset = bx . and (neg_value , align_minus_1) ; bx . add (value , offset) }
};
}
