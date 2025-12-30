// Generated macro for SmallIndexError (struct)
macro_rules! Depcrate_util_primitivesSmallIndexError {
() => {
// Module: crate::util::primitives
// Provides: {"SmallIndexError"}
// Dependencies: {}
# [doc = " This error occurs when a small index could not be constructed."] # [doc = ""] # [doc = " This occurs when given an integer exceeding the maximum small index value."] # [doc = ""] # [doc = " When the `std` feature is enabled, this implements the `Error` trait."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct SmallIndexError { attempted : u64 , }
};
}
