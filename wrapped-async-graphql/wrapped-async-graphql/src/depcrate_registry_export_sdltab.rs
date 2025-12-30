// Generated macro for tab (function)
macro_rules! Depcrate_registry_export_sdltab {
() => {
// Module: crate::registry::export_sdl
// Provides: {"tab"}
// Dependencies: {}
fn tab (options : & SDLExportOptions) -> String { if options . use_space_ident { " " . repeat (options . indent_width . into ()) } else { "\t" . to_string () } }
};
}
