macro_rules! deps {
    () => {
        InMemoryDir!();
    };
}

macro_rules! validate_upload_with_contents {
    () => {
        deps!();
        # [doc = " Check the `cargo publish` API call, with file contents"] # [track_caller] pub fn validate_upload_with_contents (expected_json : & str , expected_crate_name : & str , expected_files : & [& str] , expected_contents : impl Into < InMemoryDir > ,) { let new_path = registry :: api_path () . join ("api/v1/crates/new") ; _validate_upload (& new_path , expected_json , expected_crate_name , expected_files , expected_contents ,) ; }
    };
}

validate_upload_with_contents!();