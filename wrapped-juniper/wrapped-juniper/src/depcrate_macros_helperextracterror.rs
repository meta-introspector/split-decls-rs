// Generated macro for ExtractError (trait)
macro_rules! Depcrate_macros_helperExtractError {
() => {
// Module: crate::macros::helper
// Provides: {"ExtractError"}
// Dependencies: {}
# [doc = " This trait is used by [`graphql_scalar`] macro to retrieve [`Error`] type from a [`Result`]."] # [doc = ""] # [doc = " [`Error`]: Result::Error"] # [doc = " [`graphql_scalar`]: macro@crate::graphql_scalar"] pub trait ExtractError { # [doc = " Extracted [`Error`] type of this [`Result`]."] # [doc = ""] # [doc = " [`Error`]: Result::Error"] type Error ; }
};
}
