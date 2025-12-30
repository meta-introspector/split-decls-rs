// Generated macro for ParseErrorKind (enum)
macro_rules! Depcrate_parserParseErrorKind {
() => {
// Module: crate::parser
// Provides: {"ParseErrorKind"}
// Dependencies: {}
# [derive (Debug)] # [allow (clippy :: enum_variant_names)] enum ParseErrorKind { EmptyFlag , InvalidNamedFlag { # [cfg (not (feature = "std"))] got : () , # [cfg (feature = "std")] got : String , } , InvalidHexFlag { # [cfg (not (feature = "std"))] got : () , # [cfg (feature = "std")] got : String , } , }
};
}
