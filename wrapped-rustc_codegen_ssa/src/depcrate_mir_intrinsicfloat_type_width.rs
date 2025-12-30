// Generated macro for float_type_width (function)
macro_rules! Depcrate_mir_intrinsicfloat_type_width {
() => {
// Module: crate::mir::intrinsic
// Provides: {"float_type_width"}
// Dependencies: {}
fn float_type_width (ty : Ty < '_ >) -> Option < u64 > { match ty . kind () { ty :: Float (t) => Some (t . bit_width ()) , _ => None , } }
};
}
