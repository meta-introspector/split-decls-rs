// Generated macro for other_1964 (other)
macro_rules! Depcrate_vecother_1964 {
() => {
// Module: crate::vec
// Provides: {"other_1964"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] # [rustc_const_unstable (feature = "const_default" , issue = "143894")] impl < T > const Default for Vec < T > { # [doc = " Creates an empty `Vec<T>`."] # [doc = ""] # [doc = " The vector will not allocate until elements are pushed onto it."] fn default () -> Vec < T > { Vec :: new () } }
};
}
