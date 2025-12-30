// Generated macro for eq_case (function)
macro_rules! Depcrate_dbeq_case {
() => {
// Module: crate::db
// Provides: {"eq_case"}
// Dependencies: {}
# [doc = " A const implementation of case-insensitive ASCII equals."] const fn eq_case (lhs : & [u8] , rhs : & [u8]) -> bool { if lhs . len () != rhs . len () { return false ; } let mut i = 0usize ; while i < lhs . len () { if ! lhs [i] . eq_ignore_ascii_case (& rhs [i]) { return false ; } i += 1 ; } true }
};
}
