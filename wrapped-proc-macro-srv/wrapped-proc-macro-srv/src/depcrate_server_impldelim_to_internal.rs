// Generated macro for delim_to_internal (function)
macro_rules! Depcrate_server_impldelim_to_internal {
() => {
// Module: crate::server_impl
// Provides: {"delim_to_internal"}
// Dependencies: {}
fn delim_to_internal < S > (d : proc_macro :: Delimiter , span : bridge :: DelimSpan < S >) -> tt :: Delimiter < S > { let kind = match d { proc_macro :: Delimiter :: Parenthesis => tt :: DelimiterKind :: Parenthesis , proc_macro :: Delimiter :: Brace => tt :: DelimiterKind :: Brace , proc_macro :: Delimiter :: Bracket => tt :: DelimiterKind :: Bracket , proc_macro :: Delimiter :: None => tt :: DelimiterKind :: Invisible , } ; tt :: Delimiter { open : span . open , close : span . close , kind } }
};
}
