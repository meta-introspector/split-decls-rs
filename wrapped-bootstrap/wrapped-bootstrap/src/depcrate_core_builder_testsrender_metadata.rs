// Generated macro for render_metadata (function)
macro_rules! Depcrate_core_builder_testsrender_metadata {
() => {
// Module: crate::core::builder::tests
// Provides: {"render_metadata"}
// Dependencies: {}
fn render_metadata (metadata : & StepMetadata , config : & RenderConfig) -> String { let mut record = format ! ("[{}] " , metadata . kind . as_str ()) ; if let Some (compiler) = metadata . built_by { write ! (record , "{} -> " , render_compiler (compiler , config)) ; } let stage = metadata . get_stage () . map (| stage | format ! ("{stage} ")) . unwrap_or_default () ; write ! (record , "{} {stage}<{}>" , metadata . name , normalize_target (metadata . target , config)) ; if let Some (metadata) = & metadata . metadata { write ! (record , " {metadata}") ; } record }
};
}
