// Generated macro for MatchedFlags (struct)
macro_rules! Depcrate_options_parserMatchedFlags {
() => {
// Module: crate::options::parser
// Provides: {"MatchedFlags"}
// Dependencies: {}
# [derive (PartialEq , Eq , Debug)] pub struct MatchedFlags < 'args > { # [doc = " The individual flags from the user’s input, in the order they were"] # [doc = " originally given."] # [doc = ""] # [doc = " Long and short arguments need to be kept in the same vector because"] # [doc = " we usually want the one nearest the end to count, and to know this,"] # [doc = " we need to know where they are in relation to one another."] flags : Vec < (Flag , Option < & 'args OsStr >) > , # [doc = " Whether to check for duplicate or redundant arguments."] strictness : Strictness , }
};
}
