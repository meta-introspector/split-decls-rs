// Generated macro for Compare (trait)
macro_rules! Depcrate_traitsCompare {
() => {
// Module: crate::traits
// Provides: {"Compare"}
// Dependencies: {}
# [doc = " Abstracts comparison operations"] pub trait Compare < T > { # [doc = " Compares self to another value for equality"] fn compare (& self , t : T) -> CompareResult ; # [doc = " Compares self to another value for equality"] # [doc = " independently of the case."] # [doc = ""] # [doc = " Warning: for `&str`, the comparison is done"] # [doc = " by lowercasing both strings and comparing"] # [doc = " the result. This is a temporary solution until"] # [doc = " a better one appears"] fn compare_no_case (& self , t : T) -> CompareResult ; }
};
}
