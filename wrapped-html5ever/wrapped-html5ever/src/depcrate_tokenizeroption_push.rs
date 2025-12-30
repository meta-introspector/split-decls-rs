// Generated macro for option_push (function)
macro_rules! Depcrate_tokenizeroption_push {
() => {
// Module: crate::tokenizer
// Provides: {"option_push"}
// Dependencies: {}
fn option_push (opt_str : & mut Option < StrTendril > , c : char) { match * opt_str { Some (ref mut s) => s . push_char (c) , None => * opt_str = Some (StrTendril :: from_char (c)) , } }
};
}
