// Generated macro for format_possible_values (function)
macro_rules! Depcrate_renderformat_possible_values {
() => {
// Module: crate::render
// Provides: {"format_possible_values"}
// Dependencies: {}
fn format_possible_values (possibles : & Vec < & clap :: builder :: PossibleValue >) -> (Vec < String > , bool) { let mut lines = vec ! [] ; let with_help = possibles . iter () . any (| p | p . get_help () . is_some ()) ; if with_help { for value in possibles { let val_name = value . get_name () ; match value . get_help () { Some (help) => lines . push (format ! ("{val_name}: {help}")) , None => lines . push (val_name . to_string ()) , } } } else { lines . append (& mut possibles . iter () . map (| p | p . get_name () . to_string ()) . collect ()) ; } (lines , with_help) }
};
}
