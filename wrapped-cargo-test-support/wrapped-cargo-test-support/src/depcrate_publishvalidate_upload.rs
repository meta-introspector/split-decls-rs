// Generated macro for validate_upload (function)
macro_rules! Depcrate_publishvalidate_upload {
() => {
// Module: crate::publish
// Provides: {"validate_upload"}
// Dependencies: {}
# [doc = " Check the `cargo publish` API call"] # [track_caller] pub fn validate_upload (expected_json : & str , expected_crate_name : & str , expected_files : & [& str]) { let new_path = registry :: api_path () . join ("api/v1/crates/new") ; _validate_upload (& new_path , expected_json , expected_crate_name , expected_files , () ,) ; }
};
}
