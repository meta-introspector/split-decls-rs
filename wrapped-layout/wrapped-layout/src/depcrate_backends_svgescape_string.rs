// Generated macro for escape_string (function)
macro_rules! Depcrate_backends_svgescape_string {
() => {
// Module: crate::backends::svg
// Provides: {"escape_string"}
// Dependencies: {}
fn escape_string (x : & str) -> String { let mut res = String :: new () ; for c in x . chars () { match c { '&' => { res . push_str ("&amp;") ; } '<' => { res . push_str ("&lt;") ; } '>' => { res . push_str ("&gt;") ; } '"' => { res . push_str ("&quot;") ; } '\'' => { res . push_str ("&apos;") ; } _ => { res . push (c) ; } } } res }
};
}
