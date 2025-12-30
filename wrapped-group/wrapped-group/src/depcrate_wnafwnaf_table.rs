// Generated macro for wnaf_table (function)
macro_rules! Depcrate_wnafwnaf_table {
() => {
// Module: crate::wnaf
// Provides: {"wnaf_table"}
// Dependencies: {}
# [doc = " Replaces the contents of `table` with a w-NAF window table for the given window size."] pub (crate) fn wnaf_table < G : Group > (table : & mut Vec < G > , mut base : G , window : usize) { table . truncate (0) ; table . reserve (1 << (window - 1)) ; let dbl = base . double () ; for _ in 0 .. (1 << (window - 1)) { table . push (base) ; base . add_assign (& dbl) ; } }
};
}
