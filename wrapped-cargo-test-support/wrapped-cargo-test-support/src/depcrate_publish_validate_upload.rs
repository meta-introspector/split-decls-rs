// Generated macro for _validate_upload (function)
macro_rules! Depcrate_publish_validate_upload {
() => {
// Module: crate::publish
// Provides: {"_validate_upload"}
// Dependencies: {}
# [track_caller] fn _validate_upload (new_path : & Path , expected_json : & str , expected_crate_name : & str , expected_files : & [& str] , expected_contents : impl Into < InMemoryDir > ,) { let (actual_json , krate_bytes) = read_new_post (new_path) ; snapbox :: assert_data_eq ! (actual_json , expected_json . is_json ()) ; validate_crate_contents (& krate_bytes [..] , expected_crate_name , expected_files , expected_contents ,) ; }
};
}
