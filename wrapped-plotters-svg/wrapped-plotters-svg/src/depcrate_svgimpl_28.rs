// Generated macro for impl_28 (impl)
macro_rules! Depcrate_svgimpl_28 {
() => {
// Module: crate::svg
// Provides: {"impl_28"}
// Dependencies: {}
impl < T : FormatEscaped > FormatEscaped for Option < T > { fn format_escaped (buf : & mut String , opt : Option < T >) { match opt { None => { FormatEscaped :: format_escaped (buf , "none") ; } Some (x) => { FormatEscaped :: format_escaped (buf , x) ; } } } }
};
}
