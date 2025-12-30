// Generated macro for DatasetValidationReport (struct)
macro_rules! Depcrate_parquet_validatorDatasetValidationReport {
() => {
// Module: crate::parquet_validator
// Provides: {"DatasetValidationReport"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct DatasetValidationReport { pub dataset_name : String , pub total_files : usize , pub total_rows : usize , pub total_size_bytes : u64 , pub splits : HashMap < String , SplitValidationInfo > , pub schema_consistency : bool , pub validation_result : ValidationResult , pub sample_records : Vec < HashMap < String , String > > , }
};
}
