// Generated macro for str_to_ascii_lower_eq_str (function)
macro_rules! Depcratestr_to_ascii_lower_eq_str {
() => {
// Module: crate
// Provides: {"str_to_ascii_lower_eq_str"}
// Dependencies: {}
fn str_to_ascii_lower_eq_str (a : & str , b : & str) -> bool { a . len () == b . len () && a . bytes () . zip (b . bytes ()) . all (| (a , b) | { let a_to_ascii_lower = a | (((b'A' <= a && a <= b'Z') as u8) << 5) ; a_to_ascii_lower == b }) }
};
}
