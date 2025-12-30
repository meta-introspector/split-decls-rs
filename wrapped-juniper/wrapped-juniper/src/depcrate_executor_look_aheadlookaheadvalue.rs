// Generated macro for LookAheadValue (enum)
macro_rules! Depcrate_executor_look_aheadLookAheadValue {
() => {
// Module: crate::executor::look_ahead
// Provides: {"LookAheadValue"}
// Dependencies: {}
# [doc = " JSON-like value performing [look-ahead][0] operations on an executed GraphQL query."] # [doc = ""] # [doc = " In contrast to an [`InputValue`], these values do only contain constants, meaning that GraphQL"] # [doc = " variables get automatically resolved."] # [doc = ""] # [doc = " [0]: https://en.wikipedia.org/wiki/Look-ahead_(backtracking)"] # [expect (missing_docs , reason = "self-explanatory")] # [derive (Clone , Debug , PartialEq)] # [must_use] pub enum LookAheadValue < 'a , S : ScalarValue + 'a > { Null , Scalar (& 'a S) , Enum (& 'a str) , List (LookAheadList < 'a , S >) , Object (LookAheadObject < 'a , S >) , }
};
}
