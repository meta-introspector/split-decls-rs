// Generated macro for delim_to_external (function)
macro_rules! Depcrate_server_impldelim_to_external {
() => {
// Module: crate::server_impl
// Provides: {"delim_to_external"}
// Dependencies: {}
fn delim_to_external < S > (d : tt :: Delimiter < S >) -> proc_macro :: Delimiter { match d . kind { tt :: DelimiterKind :: Parenthesis => proc_macro :: Delimiter :: Parenthesis , tt :: DelimiterKind :: Brace => proc_macro :: Delimiter :: Brace , tt :: DelimiterKind :: Bracket => proc_macro :: Delimiter :: Bracket , tt :: DelimiterKind :: Invisible => proc_macro :: Delimiter :: None , } }
};
}
