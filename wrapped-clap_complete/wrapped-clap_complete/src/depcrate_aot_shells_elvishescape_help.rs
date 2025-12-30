// Generated macro for escape_help (function)
macro_rules! Depcrate_aot_shells_elvishescape_help {
() => {
// Module: crate::aot::shells::elvish
// Provides: {"escape_help"}
// Dependencies: {}
fn escape_help < T : ToString > (help : Option < & StyledStr > , data : T) -> String { match help { Some (help) => escape_string (& help . to_string () . replace ('\n' , " ")) , _ => data . to_string () , } }
};
}
