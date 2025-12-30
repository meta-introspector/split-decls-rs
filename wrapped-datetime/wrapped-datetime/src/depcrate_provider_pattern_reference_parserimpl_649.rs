// Generated macro for impl_649 (impl)
macro_rules! Depcrate_provider_pattern_reference_parserimpl_649 {
() => {
// Module: crate::provider::pattern::reference::parser
// Provides: {"impl_649"}
// Dependencies: {}
impl Segment { fn finish (self , result : & mut Vec < PatternItem >) -> Result < () , PatternError > { match self { Self :: Symbol (v) => v . finish (result) , Self :: SecondSymbol (v) => v . finish (result) , Self :: Literal (v) => v . finish (result) , Self :: SymbolAlias (v) => v . finish (result) , } } fn finish_generic (self , result : & mut Vec < GenericPatternItem >) -> Result < () , PatternError > { match self { Self :: Symbol (_) => unreachable ! ("no symbols in generic pattern") , Self :: SecondSymbol (_) => unreachable ! ("no symbols in generic pattern") , Self :: Literal (v) => v . finish_generic (result) , Self :: SymbolAlias (_) => unreachable ! ("no symbols in generic pattern") , } } }
};
}
