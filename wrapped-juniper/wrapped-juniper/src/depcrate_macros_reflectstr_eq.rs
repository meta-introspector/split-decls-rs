// Generated macro for str_eq (function)
macro_rules! Depcrate_macros_reflectstr_eq {
() => {
// Module: crate::macros::reflect
// Provides: {"str_eq"}
// Dependencies: {}
# [doc = " Compares strings in a `const` context."] # [doc = ""] # [doc = " As there is no `const impl Trait` and `l == r` calls [`Eq`], we have to"] # [doc = " write custom comparison function."] # [doc = ""] # [doc = " [`Eq`]: std::cmp::Eq"] pub const fn str_eq (l : & str , r : & str) -> bool { let (l , r) = (l . as_bytes () , r . as_bytes ()) ; if l . len () != r . len () { return false ; } let mut i = 0 ; while i < l . len () { if l [i] != r [i] { return false ; } i += 1 ; } true }
};
}
