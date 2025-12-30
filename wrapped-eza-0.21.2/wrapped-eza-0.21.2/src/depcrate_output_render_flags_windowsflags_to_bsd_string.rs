// Generated macro for flags_to_bsd_string (function)
macro_rules! Depcrate_output_render_flags_windowsflags_to_bsd_string {
() => {
// Module: crate::output::render::flags_windows
// Provides: {"flags_to_bsd_string"}
// Dependencies: {}
fn flags_to_bsd_string (flags : f :: flag_t) -> String { let mut result = Vec :: new () ; for attribute in & ATTRIBUTES { if attribute . flag & flags != 0 { result . push (attribute . name) ; } } if result . is_empty () { "-" . to_string () } else { result . join ("-") } }
};
}
