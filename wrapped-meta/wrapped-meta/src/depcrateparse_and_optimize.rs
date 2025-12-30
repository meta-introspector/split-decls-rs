// Generated macro for parse_and_optimize (function)
macro_rules! Depcrateparse_and_optimize {
() => {
// Module: crate
// Provides: {"parse_and_optimize"}
// Dependencies: {}
# [doc = " Parses, validates, processes and optimizes the provided grammar."] pub fn parse_and_optimize (grammar : & str ,) -> Result < UsedBuiltinAndOptimized < '_ > , Vec < Error < parser :: Rule > > > { let pairs = match parser :: parse (parser :: Rule :: grammar_rules , grammar) { Ok (pairs) => Ok (pairs) , Err (error) => Err (vec ! [error]) , } ? ; let defaults = validator :: validate_pairs (pairs . clone ()) ? ; let ast = parser :: consume_rules (pairs) ? ; Ok ((defaults , optimizer :: optimize (ast))) }
};
}
