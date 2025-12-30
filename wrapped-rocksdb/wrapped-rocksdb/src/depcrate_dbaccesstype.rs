// Generated macro for AccessType (enum)
macro_rules! Depcrate_dbAccessType {
() => {
// Module: crate::db
// Provides: {"AccessType"}
// Dependencies: {}
enum AccessType < 'a > { ReadWrite , ReadOnly { error_if_log_file_exist : bool } , Secondary { secondary_path : & 'a Path } , WithTTL { ttl : Duration } , }
};
}
