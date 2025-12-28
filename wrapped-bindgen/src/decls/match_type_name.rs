macro_rules! match_type_name {
    () => {
        fn match_type_name (rule : & str , namespace : & str , name : & str) -> bool { if rule . len () <= namespace . len () { return namespace . starts_with (rule) ; } if ! rule . starts_with (namespace) { return false ; } if rule . as_bytes () [namespace . len ()] != b'.' { return false ; } name == & rule [namespace . len () + 1 ..] }
    };
}

match_type_name!();