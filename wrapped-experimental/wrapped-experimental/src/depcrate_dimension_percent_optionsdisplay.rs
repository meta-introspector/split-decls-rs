// Generated macro for Display (enum)
macro_rules! Depcrate_dimension_percent_optionsDisplay {
() => {
// Module: crate::dimension::percent::options
// Provides: {"Display"}
// Dependencies: {}
# [derive (Debug , Eq , PartialEq , Clone , Copy)] # [non_exhaustive] pub enum Display { # [doc = " Format the Percent to display with the standard formatting for the locale."] # [doc = ""] # [doc = " For example 123 -> 123% in en-US."] Standard , # [doc = " Format the Percent to display as an approximate value."] # [doc = ""] # [doc = " For example 123 -> ~123% in en-US."] Approximate , # [doc = " Format the Percent to display with an explicit sign."] # [doc = ""] # [doc = " For example 123 -> +123% in en-US."] ExplicitSign , }
};
}
