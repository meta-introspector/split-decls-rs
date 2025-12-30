// Generated macro for write_or_print (function)
macro_rules! Depcrate_prettywrite_or_print {
() => {
// Module: crate::pretty
// Provides: {"write_or_print"}
// Dependencies: {}
fn write_or_print (out : & str , sess : & Session) { sess . io . output_file . as_ref () . unwrap_or (& OutFileName :: Stdout) . overwrite (out , sess) ; }
};
}
