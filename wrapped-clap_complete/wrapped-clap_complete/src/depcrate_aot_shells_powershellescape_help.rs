// Generated macro for escape_help (function)
macro_rules! Depcrate_aot_shells_powershellescape_help {
() => {
// Module: crate::aot::shells::powershell
// Provides: {"escape_help"}
// Dependencies: {}
fn escape_help < T : ToString > (help : Option < & StyledStr > , data : T) -> String { if let Some (help) = help { let help_str = help . to_string () ; if ! help_str . is_empty () { return escape_string (& help_str . replace ('\n' , " ")) ; } } data . to_string () }
};
}
