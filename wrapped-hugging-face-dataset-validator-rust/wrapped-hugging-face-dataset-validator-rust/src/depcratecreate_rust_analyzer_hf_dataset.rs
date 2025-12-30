// Generated macro for create_rust_analyzer_hf_dataset (function)
macro_rules! Depcratecreate_rust_analyzer_hf_dataset {
() => {
// Module: crate
// Provides: {"create_rust_analyzer_hf_dataset"}
// Dependencies: {}
# [doc = " Create HF dataset from rust-analyzer records"] fn create_rust_analyzer_hf_dataset (records : Vec < rust_analyzer_extractor :: RustAnalyzerRecord > , output_path : & str) -> Result < () , ValidationError > { use std :: collections :: HashMap ; use std :: fs ; println ! ("📦 Creating HF dataset with {} records..." , records . len ()) ; let output_dir = Path :: new (output_path) ; fs :: create_dir_all (output_dir) . map_err (| e | ValidationError :: ProcessingError (format ! ("Failed to create output directory: {}" , e))) ? ; let mut phase_groups : HashMap < String , Vec < _ > > = HashMap :: new () ; for record in records { phase_groups . entry (record . phase . clone ()) . or_default () . push (record) ; } println ! ("📊 Found {} different phases" , phase_groups . len ()) ; for (phase , phase_records) in phase_groups { println ! ("  📝 Creating dataset for phase '{}' with {} records" , phase , phase_records . len ()) ; let phase_dir = output_dir . join (format ! ("{}-phase" , phase)) ; fs :: create_dir_all (& phase_dir) . map_err (| e | ValidationError :: ProcessingError (format ! ("Failed to create phase directory: {}" , e))) ? ; let json_file = phase_dir . join ("data.json") ; let json_data = serde_json :: to_string_pretty (& phase_records) . map_err (| e | ValidationError :: ProcessingError (format ! ("Failed to serialize records: {}" , e))) ? ; fs :: write (& json_file , json_data) . map_err (| e | ValidationError :: ProcessingError (format ! ("Failed to write JSON file: {}" , e))) ? ; let readme_content = format ! ("# Rust-Analyzer {} Phase Dataset\n\n\
            This dataset contains {} records from the {} processing phase.\n\n\
            ## Schema\n\
            - `id`: Unique identifier for the record\n\
            - `file_path`: Path to the source file\n\
            - `line`, `column`: Location in the source file\n\
            - `phase`: Processing phase name\n\
            - `element_type`: Type of code element (function, struct, etc.)\n\
            - `source_snippet`: Source code snippet\n\
            - Various phase-specific data fields\n" , phase , phase_records . len () , phase) ; fs :: write (phase_dir . join ("README.md") , readme_content) . map_err (| e | ValidationError :: ProcessingError (format ! ("Failed to write README: {}" , e))) ? ; } Ok (()) }
};
}
