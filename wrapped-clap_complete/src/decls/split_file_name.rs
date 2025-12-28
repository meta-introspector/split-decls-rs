macro_rules! split_file_name {
    () => {
        fn split_file_name (path : & std :: path :: Path) -> (& std :: path :: Path , & OsStr) { if path_has_name (path) { (path . parent () . unwrap_or_else (| | std :: path :: Path :: new ("")) , path . file_name () . expect ("not called with `..`") ,) } else { (path , Default :: default ()) } }
    };
}

split_file_name!()