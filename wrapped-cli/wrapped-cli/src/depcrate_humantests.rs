// Generated macro for tests (module)
macro_rules! Depcrate_humantests {
() => {
// Module: crate::human
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn suffix_none () { let x = parse_human_readable_size ("123") . unwrap () ; assert_eq ! (123 , x) ; } # [test] fn suffix_k () { let x = parse_human_readable_size ("123K") . unwrap () ; assert_eq ! (123 * (1 << 10) , x) ; } # [test] fn suffix_m () { let x = parse_human_readable_size ("123M") . unwrap () ; assert_eq ! (123 * (1 << 20) , x) ; } # [test] fn suffix_g () { let x = parse_human_readable_size ("123G") . unwrap () ; assert_eq ! (123 * (1 << 30) , x) ; } # [test] fn invalid_empty () { assert ! (parse_human_readable_size ("") . is_err ()) ; } # [test] fn invalid_non_digit () { assert ! (parse_human_readable_size ("a") . is_err ()) ; } # [test] fn invalid_overflow () { assert ! (parse_human_readable_size ("9999999999999999G") . is_err ()) ; } # [test] fn invalid_suffix () { assert ! (parse_human_readable_size ("123T") . is_err ()) ; } }
};
}
