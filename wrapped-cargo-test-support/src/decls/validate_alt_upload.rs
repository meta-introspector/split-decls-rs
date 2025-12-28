macro_rules! validate_alt_upload {
    () => {
        # [doc = " Check the `cargo publish` API call to the alternative test registry"] # [track_caller] pub fn validate_alt_upload (expected_json : & str , expected_crate_name : & str , expected_files : & [& str] ,) { let new_path = alt_api_path () . join ("api/v1/crates/new") ; _validate_upload (& new_path , expected_json , expected_crate_name , expected_files , () ,) ; }
    };
}

validate_alt_upload!()