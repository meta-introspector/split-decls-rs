// Generated macro for parse_input (function)
macro_rules! Depcrateparse_input {
() => {
// Module: crate
// Provides: {"parse_input"}
// Dependencies: {}
fn parse_input (input : TokenStream) -> syn :: Result < (TokenStream2 , Vec < syn :: Stmt >) > { let mut input = TokenStream2 :: from (input) . into_iter () ; let crate_path = match input . next () . unwrap () { TokenTree :: Group (group) => group . stream () , _ => panic ! () , } ; let stmts = syn :: Block :: parse_within . parse2 (replace_for_await (input)) ? ; Ok ((crate_path , stmts)) }
};
}
