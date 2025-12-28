macro_rules! deps {
    () => {
        InMemoryDir!();
    };
}

macro_rules! validate_crate_contents {
    () => {
        deps!();
        # [doc = " Checks the contents of a `.crate` file."] # [doc = ""] # [doc = " - `expected_crate_name` should be something like `foo-0.0.1.crate`."] # [doc = " - `expected_files` should be a complete list of files in the crate"] # [doc = "   (relative to `expected_crate_name`)."] # [doc = " - `expected_contents` should be a list of `(file_name, contents)` tuples"] # [doc = "   to validate the contents of the given file. Only the listed files will"] # [doc = "   be checked (others will be ignored)."] # [track_caller] pub fn validate_crate_contents (reader : impl Read , expected_crate_name : & str , expected_files : & [& str] , expected_contents : impl Into < InMemoryDir > ,) { let expected_contents = expected_contents . into () ; validate_crate_contents_ (reader , expected_crate_name , expected_files , expected_contents ,) }
    };
}

validate_crate_contents!()