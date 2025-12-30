// Generated macro for sol_str_to_lamports (function)
macro_rules! Depcratesol_str_to_lamports {
() => {
// Module: crate
// Provides: {"sol_str_to_lamports"}
// Dependencies: {}
# [doc = " Convert native tokens (SOL) into fractional native tokens (lamports)"] pub fn sol_str_to_lamports (sol_str : & str) -> Option < u64 > { if sol_str == "." { None } else { let (sol , lamports) = sol_str . split_once ('.') . unwrap_or ((sol_str , "")) ; let sol = if sol . is_empty () { 0 } else { sol . parse :: < u64 > () . ok () ? } ; let lamports = if lamports . is_empty () { 0 } else { format ! ("{lamports:0<9}") [.. SOL_DECIMALS] . parse () . ok () ? } ; LAMPORTS_PER_SOL . checked_mul (sol) . and_then (| x | x . checked_add (lamports)) } }
};
}
