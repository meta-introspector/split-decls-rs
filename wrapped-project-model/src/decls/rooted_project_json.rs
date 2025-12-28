macro_rules! deps {
    () => {
        ProjectJson!();
        ProjectJsonData!();
    };
}

macro_rules! rooted_project_json {
    () => {
        deps!();
        fn rooted_project_json (data : ProjectJsonData) -> ProjectJson { let mut root = "$ROOT$" . to_owned () ; replace_root (& mut root , true) ; let path = Utf8Path :: new (& root) ; let base = AbsPath :: assert (path) ; ProjectJson :: new (None , base , data) }
    };
}

rooted_project_json!();