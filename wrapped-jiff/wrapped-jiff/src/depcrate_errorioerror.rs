// Generated macro for IOError (struct)
macro_rules! Depcrate_errorIOError {
() => {
// Module: crate::error
// Provides: {"IOError"}
// Dependencies: {}
# [doc = " A `std::io::Error`."] # [doc = ""] # [doc = " This type is itself always available, even when the `std` feature is not"] # [doc = " enabled. When `std` is not enabled, a value of this type can never be"] # [doc = " constructed."] # [doc = ""] # [doc = " Otherwise, this type is a simple wrapper around `std::io::Error`. Its"] # [doc = " purpose is to encapsulate the conditional compilation based on the `std`"] # [doc = " feature."] # [cfg_attr (not (feature = "alloc") , derive (Clone))] struct IOError { # [cfg (feature = "std")] err : std :: io :: Error , }
};
}
