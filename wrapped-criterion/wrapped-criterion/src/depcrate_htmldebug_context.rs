// Generated macro for debug_context (function)
macro_rules! Depcrate_htmldebug_context {
() => {
// Module: crate::html
// Provides: {"debug_context"}
// Dependencies: {}
fn debug_context < S : Serialize > (path : & Path , context : & S) { if crate :: debug_enabled () { let mut context_path = PathBuf :: from (path) ; context_path . set_extension ("json") ; println ! ("Writing report context to {:?}" , context_path) ; let result = fs :: save (context , & context_path) ; if let Err (e) = result { error ! ("Failed to write report context debug output: {}" , e) ; } } }
};
}
