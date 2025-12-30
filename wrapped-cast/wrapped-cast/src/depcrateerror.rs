// Generated macro for Error (enum)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Cast errors"] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum Error { # [doc = " Infinite value casted to a type that can only represent finite values"] Infinite , # [doc = " NaN value casted to a type that can't represent a NaN value"] NaN , # [doc = " Source value is greater than the maximum value that the destination type"] # [doc = " can hold"] Overflow , # [doc = " Source value is smaller than the minimum value that the destination type"] # [doc = " can hold"] Underflow , }
};
}
