// Generated macro for write_deprecated (function)
macro_rules! Depcrate_registry_export_sdlwrite_deprecated {
() => {
// Module: crate::registry::export_sdl
// Provides: {"write_deprecated"}
// Dependencies: {}
fn write_deprecated (sdl : & mut String , deprecation : & Deprecation) { if let Deprecation :: Deprecated { reason } = deprecation { let _ = match reason { Some (reason) => write ! (sdl , " @deprecated(reason: \"{}\")" , escape_string (reason)) . ok () , None => write ! (sdl , " @deprecated") . ok () , } ; } }
};
}
