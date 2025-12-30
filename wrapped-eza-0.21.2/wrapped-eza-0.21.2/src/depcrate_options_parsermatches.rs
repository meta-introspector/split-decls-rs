// Generated macro for Matches (struct)
macro_rules! Depcrate_options_parserMatches {
() => {
// Module: crate::options::parser
// Provides: {"Matches"}
// Dependencies: {}
# [doc = " The **matches** are the result of parsing the user’s command-line strings."] # [derive (PartialEq , Eq , Debug)] pub struct Matches < 'args > { # [doc = " The flags that were parsed from the user’s input."] pub flags : MatchedFlags < 'args > , # [doc = " All the strings that weren’t matched as arguments, as well as anything"] # [doc = " after the special “--” string."] pub frees : Vec < & 'args OsStr > , }
};
}
