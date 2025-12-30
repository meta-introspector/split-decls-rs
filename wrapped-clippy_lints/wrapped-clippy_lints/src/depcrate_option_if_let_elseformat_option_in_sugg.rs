// Generated macro for format_option_in_sugg (function)
macro_rules! Depcrate_option_if_let_elseformat_option_in_sugg {
() => {
// Module: crate::option_if_let_else
// Provides: {"format_option_in_sugg"}
// Dependencies: {}
fn format_option_in_sugg (cond_sugg : Sugg < '_ > , as_ref : bool , as_mut : bool) -> String { format ! ("{}{}" , cond_sugg . maybe_paren () , if as_mut { ".as_mut()" } else if as_ref { ".as_ref()" } else { "" }) }
};
}
