// Generated macro for SplitValidationInfo (struct)
macro_rules! Depcrate_parquet_validatorSplitValidationInfo {
() => {
// Module: crate::parquet_validator
// Provides: {"SplitValidationInfo"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct SplitValidationInfo { pub split_name : String , pub num_files : usize , pub num_rows : usize , pub size_bytes : u64 , pub files : Vec < ParquetFileInfo > , }
};
}
