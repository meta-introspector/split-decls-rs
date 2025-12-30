// Generated macro for validate_parquet_dataset (function)
macro_rules! Depcrate_parquet_validatorvalidate_parquet_dataset {
() => {
// Module: crate::parquet_validator
// Provides: {"validate_parquet_dataset"}
// Dependencies: {}
# [doc = " CLI function to validate Parquet dataset"] pub fn validate_parquet_dataset (dataset_dir : & str) -> Result < () , ValidationError > { let validator = ParquetValidator :: new (dataset_dir) ? ; let report = validator . validate_dataset () ? ; validator . print_report (& report) ; let report_path = format ! ("{}/validation_report.json" , dataset_dir) ; let report_json = serde_json :: to_string_pretty (& report) ? ; fs :: write (& report_path , report_json) . map_err (| e | ValidationError :: DataAccessError { message : format ! ("Failed to write validation report: {}" , e) , }) ? ; println ! ("\n📄 Validation report saved to: {}" , report_path) ; Ok (()) }
};
}
