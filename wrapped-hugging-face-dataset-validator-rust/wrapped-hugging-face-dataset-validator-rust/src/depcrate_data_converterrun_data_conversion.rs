// Generated macro for run_data_conversion (function)
macro_rules! Depcrate_data_converterrun_data_conversion {
() => {
// Module: crate::data_converter
// Provides: {"run_data_conversion"}
// Dependencies: {}
# [doc = " CLI function to run data conversion"] pub fn run_data_conversion (base_path : & str , command : & str , output_path : & str) -> Result < () , ValidationError > { let converter = DataConverter :: new (base_path) ? ; match command { "export-all" => { println ! ("Exporting all terms to JSONL format...") ; let count = converter . export_all_to_jsonl (output_path) ? ; println ! ("✅ Exported {} terms to {}" , count , output_path) ; } "export-stats" => { println ! ("Exporting dataset statistics...") ; converter . export_statistics (output_path) ? ; println ! ("✅ Exported statistics to {}" , output_path) ; } "create-sample" => { println ! ("Creating sample dataset...") ; let count = converter . create_sample_dataset (output_path , 1000) ? ; println ! ("✅ Created sample dataset with {} terms in {}" , count , output_path) ; } _ => { return Err (ValidationError :: InvalidEntityIdentifier { message : format ! ("Unknown conversion command: {}" , command) , }) ; } } Ok (()) }
};
}
