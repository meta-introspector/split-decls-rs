// Generated macro for option_default_values (function)
macro_rules! Depcrate_renderoption_default_values {
() => {
// Module: crate::render
// Provides: {"option_default_values"}
// Dependencies: {}
fn option_default_values (opt : & Arg) -> Option < String > { if opt . is_hide_default_value_set () || ! opt . get_num_args () . expect ("built") . takes_values () { return None ; } else if ! opt . get_default_values () . is_empty () { let values = opt . get_default_values () . iter () . map (| s | s . to_string_lossy ()) . collect :: < Vec < _ > > () . join (",") ; return Some (format ! ("[default: {values}]")) ; } None }
};
}
