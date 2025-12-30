// Generated macro for impl_26 (impl)
macro_rules! Depcrate_svgimpl_26 {
() => {
// Module: crate::svg
// Provides: {"impl_26"}
// Dependencies: {}
impl FormatEscaped for & str { fn format_escaped (buf : & mut String , s : & str) { for c in s . chars () { FormatEscaped :: format_escaped (buf , c) ; } } }
};
}
