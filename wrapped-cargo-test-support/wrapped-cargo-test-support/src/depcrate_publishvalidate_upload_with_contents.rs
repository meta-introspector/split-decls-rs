// Generated macro for validate_upload_with_contents (function)
macro_rules! Depcrate_publishvalidate_upload_with_contents {
() => {
// Module: crate::publish
// Provides: {"validate_upload_with_contents"}
// Dependencies: {}
# [doc = " Check the `cargo publish` API call, with file contents"] # [track_caller] pub fn validate_upload_with_contents (expected_json : & str , expected_crate_name : & str , expected_files : & [& str] , expected_contents : impl Into < InMemoryDir > ,) { let new_path = registry :: api_path () . join ("api/v1/crates/new") ; _validate_upload (& new_path , expected_json , expected_crate_name , expected_files , expected_contents ,) ; }
};
}
