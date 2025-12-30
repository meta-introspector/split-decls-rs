// Generated macro for impl_197 (impl)
macro_rules! Depcrate_helpers_helper_logimpl_197 {
() => {
// Module: crate::helpers::helper_log
// Provides: {"impl_197"}
// Dependencies: {}
# [cfg (not (feature = "no_logging"))] impl HelperDef for LogHelper { fn call < 'reg : 'rc , 'rc > (& self , h : & Helper < 'rc > , _ : & 'reg Registry < 'reg > , _ : & 'rc Context , _ : & mut RenderContext < 'reg , 'rc > , _ : & mut dyn Output ,) -> HelperResult { let param_to_log = h . params () . iter () . map (| p | { if let Some (relative_path) = p . relative_path () { format ! ("{}: {}" , & relative_path , p . value () . render ()) } else { p . value () . render () } }) . collect :: < Vec < String > > () . join (", ") ; let level = h . hash_get ("level") . and_then (| v | v . value () . as_str ()) . unwrap_or ("info") ; if let Ok (log_level) = Level :: from_str (level) { log ! (log_level , "{}" , param_to_log) ; } else { return Err (RenderErrorReason :: InvalidLoggingLevel (level . to_string ()) . into ()) ; } Ok (()) } }
};
}
