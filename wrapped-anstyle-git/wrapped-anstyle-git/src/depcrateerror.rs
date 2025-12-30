// Generated macro for Error (enum)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Type for errors returned by the parser."] # [derive (Debug , PartialEq , Eq)] # [non_exhaustive] pub enum Error { # [doc = " An extra color appeared after the foreground and background colors."] ExtraColor { # [doc = " Original style"] style : String , # [doc = " Extra color"] word : String , } , # [doc = " An unknown word appeared."] UnknownWord { # [doc = " Original style"] style : String , # [doc = " Unknown word"] word : String , } , }
};
}
