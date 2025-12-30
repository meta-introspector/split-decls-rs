// Generated macro for impl_645 (impl)
macro_rules! Depcrate_provider_pattern_reference_parserimpl_645 {
() => {
// Module: crate::provider::pattern::reference::parser
// Provides: {"impl_645"}
// Dependencies: {}
impl SegmentLiteral { fn finish (self , result : & mut Vec < PatternItem >) -> Result < () , PatternError > { if ! self . literal . is_empty () { result . extend (self . literal . chars () . map (PatternItem :: from)) ; } Ok (()) } fn finish_generic (self , result : & mut Vec < GenericPatternItem >) -> Result < () , PatternError > { if ! self . literal . is_empty () { result . extend (self . literal . chars () . map (GenericPatternItem :: from)) ; } Ok (()) } }
};
}
