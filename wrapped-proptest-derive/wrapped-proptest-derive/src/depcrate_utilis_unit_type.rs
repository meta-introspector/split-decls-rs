// Generated macro for is_unit_type (function)
macro_rules! Depcrate_utilis_unit_type {
() => {
// Module: crate::util
// Provides: {"is_unit_type"}
// Dependencies: {}
# [doc = " Returns true iff the given type is the literal unit type `()`."] # [doc = " This is treated the same way by `syn` as a 0-tuple."] pub fn is_unit_type < T : Borrow < syn :: Type > > (ty : T) -> bool { ty . borrow () == & parse_quote ! (()) }
};
}
