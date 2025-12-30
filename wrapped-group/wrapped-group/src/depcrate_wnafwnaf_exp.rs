// Generated macro for wnaf_exp (function)
macro_rules! Depcrate_wnafwnaf_exp {
() => {
// Module: crate::wnaf
// Provides: {"wnaf_exp"}
// Dependencies: {}
# [doc = " Performs w-NAF exponentiation with the provided window table and w-NAF form scalar."] # [doc = ""] # [doc = " This function must be provided a `table` and `wnaf` that were constructed with"] # [doc = " the same window size; otherwise, it may panic or produce invalid results."] pub (crate) fn wnaf_exp < G : Group > (table : & [G] , wnaf : & [i64]) -> G { let mut result = G :: identity () ; let mut found_one = false ; for n in wnaf . iter () . rev () { if found_one { result = result . double () ; } if * n != 0 { found_one = true ; if * n > 0 { result += & table [(n / 2) as usize] ; } else { result -= & table [((- n) / 2) as usize] ; } } } result }
};
}
