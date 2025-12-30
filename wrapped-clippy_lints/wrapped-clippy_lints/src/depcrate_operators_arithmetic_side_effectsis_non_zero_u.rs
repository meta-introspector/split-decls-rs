// Generated macro for is_non_zero_u (function)
macro_rules! Depcrate_operators_arithmetic_side_effectsis_non_zero_u {
() => {
// Module: crate::operators::arithmetic_side_effects
// Provides: {"is_non_zero_u"}
// Dependencies: {}
fn is_non_zero_u (cx : & LateContext < '_ > , ty : Ty < '_ >) -> bool { if let ty :: Adt (adt , substs) = ty . kind () && cx . tcx . is_diagnostic_item (sym :: NonZero , adt . did ()) && let int_type = substs . type_at (0) && matches ! (int_type . kind () , ty :: Uint (_)) { true } else { false } }
};
}
