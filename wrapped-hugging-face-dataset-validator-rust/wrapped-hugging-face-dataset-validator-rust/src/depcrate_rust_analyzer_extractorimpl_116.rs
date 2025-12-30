// Generated macro for impl_116 (impl)
macro_rules! Depcrate_rust_analyzer_extractorimpl_116 {
() => {
// Module: crate::rust_analyzer_extractor
// Provides: {"impl_116"}
// Dependencies: {}
impl ProcessingPhase { # [doc = " Convert phase to string representation for file naming"] pub fn as_str (& self) -> & 'static str { match self { ProcessingPhase :: Parsing => "parsing" , ProcessingPhase :: NameResolution => "name_resolution" , ProcessingPhase :: TypeInference => "type_inference" , ProcessingPhase :: HirGeneration => "hir_generation" , ProcessingPhase :: Diagnostics => "diagnostics" , ProcessingPhase :: Completions => "completions" , ProcessingPhase :: Hover => "hover" , ProcessingPhase :: GotoDefinition => "goto_definition" , ProcessingPhase :: FindReferences => "find_references" , } } }
};
}
