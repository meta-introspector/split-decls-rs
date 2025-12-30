// Generated macro for macro_definition (function)
macro_rules! Depcrate_exprmacro_definition {
() => {
// Module: crate::expr
// Provides: {"macro_definition"}
// Dependencies: {}
# [doc = " Parse and evaluate a macro definition from a list of tokens."] # [doc = ""] # [doc = " Returns the identifier for the macro and its replacement evaluated as an"] # [doc = " expression. The input should not include `#define`."] # [doc = ""] # [doc = " Returns an error if the replacement is not a valid expression, if called"] # [doc = " on a function-like macro, or if the token stream contains comments,"] # [doc = " keywords or identifiers."] pub fn macro_definition (input : & [Token]) -> CResult < '_ , (& '_ [u8] , EvalResult) > { IdentifierParser :: new (& HashMap :: new ()) . macro_definition (input) }
};
}
