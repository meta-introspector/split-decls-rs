// Generated macro for analyze_rust_project (function)
macro_rules! Depcrateanalyze_rust_project {
() => {
// Module: crate
// Provides: {"analyze_rust_project"}
// Dependencies: {}
# [doc = " Analyze a Rust project with all processing phases"] fn analyze_rust_project (project_path : & str , output_path : & str) -> Result < () , ValidationError > { println ! ("🔍 Analyzing Rust project: {}" , project_path) ; println ! ("📁 Output directory: {}" , output_path) ; let project_path = Path :: new (project_path) ; if ! project_path . exists () { return Err (ValidationError :: InvalidInput (format ! ("Project path does not exist: {}" , project_path . display ()))) ; } let mut extractor = RustAnalyzerExtractor :: new () . map_err (| e | ValidationError :: ProcessingError (format ! ("Failed to create rust-analyzer extractor: {}" , e))) ? ; let phases = vec ! [ProcessingPhase :: Parsing , ProcessingPhase :: NameResolution , ProcessingPhase :: TypeInference , ProcessingPhase :: HirGeneration , ProcessingPhase :: Diagnostics ,] ; println ! ("🚀 Processing {} phases..." , phases . len ()) ; let records = extractor . process_codebase (project_path , & phases) . map_err (| e | ValidationError :: ProcessingError (format ! ("Failed to process codebase: {}" , e))) ? ; println ! ("✅ Generated {} records from rust-analyzer processing" , records . len ()) ; create_rust_analyzer_hf_dataset (records , output_path) ? ; println ! ("🎉 Successfully created rust-analyzer datasets in: {}" , output_path) ; Ok (()) }
};
}
