// Generated macro for generate_hf_dataset (function)
macro_rules! Depcrategenerate_hf_dataset {
() => {
// Module: crate
// Provides: {"generate_hf_dataset"}
// Dependencies: {}
# [doc = " Generate HuggingFace dataset with Parquet files ready for Git LFS"] fn generate_hf_dataset (project_path : & str , output_path : & str) -> Result < () , ValidationError > { println ! ("🔍 Generating HuggingFace dataset from Rust project: {}" , project_path) ; println ! ("📁 Output directory: {}" , output_path) ; let project_path = Path :: new (project_path) ; if ! project_path . exists () { return Err (ValidationError :: InvalidInput (format ! ("Project path does not exist: {}" , project_path . display ()))) ; } let mut extractor = RustAnalyzerExtractor :: new () . map_err (| e | ValidationError :: ProcessingError (format ! ("Failed to create rust-analyzer extractor: {}" , e))) ? ; let phases = vec ! [ProcessingPhase :: Parsing , ProcessingPhase :: NameResolution , ProcessingPhase :: TypeInference ,] ; println ! ("🚀 Processing {} phases and generating Parquet files..." , phases . len ()) ; let output_dir = Path :: new (output_path) ; extractor . process_codebase_to_parquet (project_path , & phases , output_dir) . map_err (| e | ValidationError :: ProcessingError (format ! ("Failed to generate Parquet files: {}" , e))) ? ; create_repository_files (output_dir , project_path) ? ; println ! ("🎉 Successfully generated HuggingFace dataset with Parquet files in: {}" , output_path) ; println ! ("📦 Ready for Git LFS - all files are under 10MB") ; Ok (()) }
};
}
