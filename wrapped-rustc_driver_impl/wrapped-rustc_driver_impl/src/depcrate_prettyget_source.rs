// Generated macro for get_source (function)
macro_rules! Depcrate_prettyget_source {
() => {
// Module: crate::pretty
// Provides: {"get_source"}
// Dependencies: {}
fn get_source (sess : & Session) -> (String , FileName) { let src_name = sess . io . input . source_name () ; let src = String :: clone (sess . source_map () . get_source_file (& src_name) . expect ("get_source_file") . src . as_ref () . expect ("src") ,) ; (src , src_name) }
};
}
