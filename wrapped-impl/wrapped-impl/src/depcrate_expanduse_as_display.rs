// Generated macro for use_as_display (function)
macro_rules! Depcrate_expanduse_as_display {
() => {
// Module: crate::expand
// Provides: {"use_as_display"}
// Dependencies: {}
fn use_as_display (needs_as_display : bool) -> Option < TokenStream > { if needs_as_display { Some (quote ! { use :: thiserror ::# private :: AsDisplay as _ ; }) } else { None } }
};
}
