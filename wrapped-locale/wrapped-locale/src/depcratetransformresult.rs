// Generated macro for TransformResult (enum)
macro_rules! DepcrateTransformResult {
() => {
// Module: crate
// Provides: {"TransformResult"}
// Dependencies: {}
# [doc = " Used to track the result of a transformation operation that potentially modifies its argument in place."] # [derive (Debug , PartialEq)] # [allow (clippy :: exhaustive_enums)] pub enum TransformResult { # [doc = " The canonicalization operation modified the locale."] Modified , # [doc = " The canonicalization operation did not modify the locale."] Unmodified , }
};
}
