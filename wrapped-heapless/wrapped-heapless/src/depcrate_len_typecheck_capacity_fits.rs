// Generated macro for check_capacity_fits (function)
macro_rules! Depcrate_len_typecheck_capacity_fits {
() => {
// Module: crate::len_type
// Provides: {"check_capacity_fits"}
// Dependencies: {}
pub const fn check_capacity_fits < LenT : LenType , const N : usize > () { assert ! (LenT :: MAX_USIZE >= N , "The capacity is larger than `LenT` can hold, increase the size of `LenT` or reduce the capacity") ; }
};
}
