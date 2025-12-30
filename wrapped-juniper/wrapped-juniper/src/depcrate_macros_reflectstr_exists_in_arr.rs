// Generated macro for str_exists_in_arr (function)
macro_rules! Depcrate_macros_reflectstr_exists_in_arr {
() => {
// Module: crate::macros::reflect
// Provides: {"str_exists_in_arr"}
// Dependencies: {}
# [doc = " Checks whether the given `val` exists in the given `arr`."] # [must_use] pub const fn str_exists_in_arr (val : & str , arr : & [& str]) -> bool { let mut i = 0 ; while i < arr . len () { if str_eq (val , arr [i]) { return true ; } i += 1 ; } false }
};
}
