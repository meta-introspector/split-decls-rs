// Generated macro for escape_dep_env (function)
macro_rules! Depcrate_passesescape_dep_env {
() => {
// Module: crate::passes
// Provides: {"escape_dep_env"}
// Dependencies: {}
fn escape_dep_env (symbol : Symbol) -> String { let s = symbol . as_str () ; let mut escaped = String :: with_capacity (s . len ()) ; for c in s . chars () { match c { '\n' => escaped . push_str (r"\n") , '\r' => escaped . push_str (r"\r") , '\\' => escaped . push_str (r"\\") , _ => escaped . push (c) , } } escaped }
};
}
