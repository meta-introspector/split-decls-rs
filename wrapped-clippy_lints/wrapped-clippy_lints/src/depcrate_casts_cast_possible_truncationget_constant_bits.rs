// Generated macro for get_constant_bits (function)
macro_rules! Depcrate_casts_cast_possible_truncationget_constant_bits {
() => {
// Module: crate::casts::cast_possible_truncation
// Provides: {"get_constant_bits"}
// Dependencies: {}
fn get_constant_bits (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> Option < u64 > { constant_int (cx , expr) . map (| c | u64 :: from (128 - c . leading_zeros ())) }
};
}
