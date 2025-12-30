// Generated macro for expr (function)
macro_rules! Depcrate_exprexpr {
() => {
// Module: crate::expr
// Provides: {"expr"}
// Dependencies: {}
# [doc = " Parse and evaluate an expression of a list of tokens."] # [doc = ""] # [doc = " Returns an error if the input is not a valid expression or if the token"] # [doc = " stream contains comments, keywords or identifiers."] pub fn expr (input : & [Token]) -> CResult < '_ , EvalResult > { IdentifierParser :: new (& HashMap :: new ()) . expr (input) }
};
}
