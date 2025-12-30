// Generated macro for Spanning (struct)
macro_rules! Depcrate_parser_utilsSpanning {
() => {
// Module: crate::parser::utils
// Provides: {"Spanning"}
// Dependencies: {}
# [doc = " Data structure used to wrap items into a [`Span`]."] # [derive (Clone , Copy , Debug , Eq , Error , Hash , PartialEq)] pub struct Spanning < T , Sp = Span > { # [doc = " Wrapped item."] # [error (source)] pub item : T , # [doc = " [`Span`] of the wrapped item."] pub span : Sp , }
};
}
