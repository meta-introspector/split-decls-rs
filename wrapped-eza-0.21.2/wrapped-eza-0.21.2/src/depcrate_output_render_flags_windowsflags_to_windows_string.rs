// Generated macro for flags_to_windows_string (function)
macro_rules! Depcrate_output_render_flags_windowsflags_to_windows_string {
() => {
// Module: crate::output::render::flags_windows
// Provides: {"flags_to_windows_string"}
// Dependencies: {}
fn flags_to_windows_string (flags : f :: flag_t) -> String { let mut result = String :: new () ; for attribute in & ATTRIBUTES { if attribute . flag & flags != 0 { result . push (attribute . abbr) ; } } if result . is_empty () { result . push ('-') ; } result }
};
}
