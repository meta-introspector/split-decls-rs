// Generated macro for analyze_rust_phases (function)
macro_rules! Depcrateanalyze_rust_phases {
() => {
// Module: crate
// Provides: {"analyze_rust_phases"}
// Dependencies: {}
# [doc = " Analyze specific Rust processing phases"] fn analyze_rust_phases (project_path : & str , phases_str : & str , output_path : & str) -> Result < () , ValidationError > { println ! ("🔍 Analyzing Rust project phases: {}" , phases_str) ; println ! ("📁 Project path: {}" , project_path) ; println ! ("📁 Output directory: {}" , output_path) ; let project_path = Path :: new (project_path) ; if ! project_path . exists () { return Err (ValidationError :: InvalidInput (format ! ("Project path does not exist: {}" , project_path . display ()))) ; } let phases = parse_phases_string (phases_str) ? ; println ! ("🎯 Selected phases: {:?}" , phases) ; let mut extractor = RustAnalyzerExtractor :: new () . map_err (| e | ValidationError :: ProcessingError (format ! ("Failed to create rust-analyzer extractor: {}" , e))) ? ; let records = extractor . process_codebase (project_path , & phases) . map_err (| e | ValidationError :: ProcessingError (format ! ("Failed to process codebase: {}" , e))) ? ; println ! ("✅ Generated {} records from {} phases" , records . len () , phases . len ()) ; create_rust_analyzer_hf_dataset (records , output_path) ? ; println ! ("🎉 Successfully created phase-specific datasets in: {}" , output_path) ; Ok (()) }
};
}
