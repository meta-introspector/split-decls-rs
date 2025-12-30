// Generated macro for tests (module)
macro_rules! Depcrate_parquet_validatortests {
() => {
// Module: crate::parquet_validator
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_parquet_validator () { let dataset_dir = "solfunmeme-hf-dataset" ; if ! Path :: new (dataset_dir) . exists () { println ! ("Skipping test - dataset not found at {}" , dataset_dir) ; return ; } let validator = ParquetValidator :: new (dataset_dir) . unwrap () ; let report = validator . validate_dataset () . unwrap () ; assert ! (report . total_files > 0) ; assert ! (report . total_rows > 0) ; assert ! (report . validation_result . viewer) ; } }
};
}
