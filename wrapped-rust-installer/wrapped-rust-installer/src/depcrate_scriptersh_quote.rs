// Generated macro for sh_quote (function)
macro_rules! Depcrate_scriptersh_quote {
() => {
// Module: crate::scripter
// Provides: {"sh_quote"}
// Dependencies: {}
fn sh_quote < T : ToString > (s : & T) -> String { format ! ("'{}'" , s . to_string () . replace ('\'' , r#"'"'"'"#)) }
};
}
