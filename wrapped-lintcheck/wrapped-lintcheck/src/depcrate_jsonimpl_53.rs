// Generated macro for impl_53 (impl)
macro_rules! Depcrate_jsonimpl_53 {
() => {
// Module: crate::json
// Provides: {"impl_53"}
// Dependencies: {}
impl Summary { fn new (lints : & [LintWarnings]) -> Self { Summary (lints . iter () . map (| lint | SummaryRow { name : lint . name . clone () , added : lint . added . len () , removed : lint . removed . len () , changed : lint . changed . len () , }) . collect () ,) } }
};
}
