// Generated macro for is_number (function)
macro_rules! Depcrate_patis_number {
() => {
// Module: crate::pat
// Provides: {"is_number"}
// Dependencies: {}
fn is_number (text : & str) -> bool { text . chars () . all (| c : char | c . is_digit (10)) }
};
}
