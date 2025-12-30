// Generated macro for Parser (trait)
macro_rules! Depcrate_parserParser {
() => {
// Module: crate::parser
// Provides: {"Parser"}
// Dependencies: {}
# [doc = " A trait with a single method that parses strings."] pub trait Parser < R : RuleType > { # [doc = " Parses a `&str` starting from `rule`."] # [allow (clippy :: perf)] fn parse (rule : R , input : & str) -> Result < Pairs < '_ , R > , Error < R > > ; }
};
}
