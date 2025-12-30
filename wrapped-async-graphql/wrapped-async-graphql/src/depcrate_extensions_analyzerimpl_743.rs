// Generated macro for impl_743 (impl)
macro_rules! Depcrate_extensions_analyzerimpl_743 {
() => {
// Module: crate::extensions::analyzer
// Provides: {"impl_743"}
// Dependencies: {}
impl ExtensionFactory for Analyzer { fn create (& self) -> Arc < dyn Extension > { Arc :: new (AnalyzerExtension :: default ()) } }
};
}
