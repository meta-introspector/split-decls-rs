// Generated macro for parse (function)
macro_rules! Depcrate_parserparse {
() => {
// Module: crate::parser
// Provides: {"parse"}
// Dependencies: {}
# [doc = " A helper that will parse using the pest grammar"] # [allow (clippy :: perf)] pub fn parse (rule : Rule , data : & str) -> Result < Pairs < '_ , Rule > , Error < Rule > > { PestParser :: parse (rule , data) }
};
}
