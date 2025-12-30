// Generated macro for escape_string (function)
macro_rules! Depcrate_registry_export_sdlescape_string {
() => {
// Module: crate::registry::export_sdl
// Provides: {"escape_string"}
// Dependencies: {}
fn escape_string (s : & str) -> String { let mut res = String :: new () ; for c in s . chars () { let ec = match c { '\\' => Some ("\\\\") , '\x08' => Some ("\\b") , '\x0c' => Some ("\\f") , '\n' => Some ("\\n") , '\r' => Some ("\\r") , '\t' => Some ("\\t") , _ => None , } ; match ec { Some (ec) => { res . write_str (ec) . ok () ; } None => { res . write_char (c) . ok () ; } } } res }
};
}
