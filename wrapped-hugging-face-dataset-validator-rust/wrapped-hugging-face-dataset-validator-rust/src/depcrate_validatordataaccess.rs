// Generated macro for DataAccess (trait)
macro_rules! Depcrate_validatorDataAccess {
() => {
// Module: crate::validator
// Provides: {"DataAccess"}
// Dependencies: {}
pub trait DataAccess : Send + Sync { fn check_successful_response (& self , kind : & str , entity : & EntityIdentifier) -> Result < bool , ValidationError > ; fn get_parquet_metadata (& self , dataset : & str , config : & str) -> Result < ParquetMetadata , ValidationError > ; fn get_split_names (& self , dataset : & str , config : & str) -> Result < Vec < String > , ValidationError > ; fn get_config_names (& self , dataset : & str) -> Result < Vec < String > , ValidationError > ; fn get_cached_validation (& self , kind : & str , entity : & EntityIdentifier) -> Result < CachedResponse , ValidationError > ; fn has_indexable_columns (& self , features : & HashMap < String , String >) -> bool ; }
};
}
