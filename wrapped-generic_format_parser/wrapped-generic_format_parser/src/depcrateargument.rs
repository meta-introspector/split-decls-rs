// Generated macro for Argument (struct)
macro_rules! DepcrateArgument {
() => {
// Module: crate
// Provides: {"Argument"}
// Dependencies: {}
# [doc = " Representation of an argument specification."] # [derive (Copy , Clone , Debug , PartialEq)] pub struct Argument < 'a > { # [doc = " Where to find this argument"] pub position : Position < 'a > , # [doc = " The span of the position indicator. Includes any whitespace in implicit"] # [doc = " positions (`{  }`)."] pub position_span : InnerSpan , # [doc = " How to format the argument"] pub format : FormatSpec < 'a > , }
};
}
