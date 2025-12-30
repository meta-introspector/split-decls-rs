// Generated macro for impl_29 (impl)
macro_rules! Depcrate_svgimpl_29 {
() => {
// Module: crate::svg
// Provides: {"impl_29"}
// Dependencies: {}
impl FormatEscaped for char { fn format_escaped (buf : & mut String , c : char) { match c { '<' => buf . push_str ("&lt;") , '>' => buf . push_str ("&gt;") , '&' => buf . push_str ("&amp;") , '"' => buf . push_str ("&quot;") , '\'' => buf . push_str ("&apos;") , other => buf . push (other) , } ; } }
};
}
