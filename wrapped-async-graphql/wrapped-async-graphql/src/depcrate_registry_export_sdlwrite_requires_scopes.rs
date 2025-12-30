// Generated macro for write_requires_scopes (function)
macro_rules! Depcrate_registry_export_sdlwrite_requires_scopes {
() => {
// Module: crate::registry::export_sdl
// Provides: {"write_requires_scopes"}
// Dependencies: {}
fn write_requires_scopes (sdl : & mut String , requires_scopes : & [String]) { write ! (sdl , " @requiresScopes(scopes: [{}])" , requires_scopes . iter () . map (| x | { "[" . to_string () + & x . split_whitespace () . map (| y | "\"" . to_string () + y + "\"") . collect ::< Vec < _ >> () . join (", ") + "]" }) . collect ::< Vec < _ >> () . join (", ")) . ok () ; }
};
}
