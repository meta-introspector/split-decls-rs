// Generated macro for anyhow_to_syn (function)
macro_rules! Depcrateanyhow_to_syn {
() => {
// Module: crate
// Provides: {"anyhow_to_syn"}
// Dependencies: {}
fn anyhow_to_syn (span : Span , err : anyhow :: Error) -> Error { let err = attach_with_context (err) ; let mut msg = err . to_string () ; for cause in err . chain () . skip (1) { msg . push_str (& format ! ("\n\nCaused by:\n  {cause}")) ; } Error :: new (span , msg) }
};
}
