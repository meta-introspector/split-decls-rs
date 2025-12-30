// Generated macro for impl_31 (impl)
macro_rules! Depcrate_svgimpl_31 {
() => {
// Module: crate::svg
// Provides: {"impl_31"}
// Dependencies: {}
impl < I : IntoIterator < Item : FormatEscaped > > FormatEscaped for FormatEscapedIter < I > { fn format_escaped (buf : & mut String , iter : FormatEscapedIter < I >) { let iter = iter . 0 . into_iter () ; for item in iter { FormatEscaped :: format_escaped (buf , item) ; } } }
};
}
