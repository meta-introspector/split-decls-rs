// Generated macro for NoError (struct)
macro_rules! DepcrateNoError {
() => {
// Module: crate
// Provides: {"NoError"}
// Dependencies: {}
# [doc = " NoError provides an error type for matchers that never produce errors."] # [doc = ""] # [doc = " This error type implements the `std::error::Error` and `std::fmt::Display`"] # [doc = " traits for use in matcher implementations that can never produce errors."] # [doc = ""] # [doc = " The `std::fmt::Debug` and `std::fmt::Display` impls for this type panics."] # [derive (Debug , Eq , PartialEq)] pub struct NoError (()) ;
};
}
