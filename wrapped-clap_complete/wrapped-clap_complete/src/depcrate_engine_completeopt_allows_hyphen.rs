// Generated macro for opt_allows_hyphen (function)
macro_rules! Depcrate_engine_completeopt_allows_hyphen {
() => {
// Module: crate::engine::complete
// Provides: {"opt_allows_hyphen"}
// Dependencies: {}
fn opt_allows_hyphen (state : & ParseState < '_ > , arg : & clap_lex :: ParsedArg < '_ >) -> bool { let val = arg . to_value_os () ; if val . starts_with ("-") { if let ParseState :: Opt ((opt , _)) = state { return opt . is_allow_hyphen_values_set () ; } } false }
};
}
