macro_rules! hash_path_arguments {
    () => {
        fn hash_path_arguments (path : & Path) -> Option < u64 > { let mut hasher = DefaultHasher :: new () ; let mut is_some = false ; for arguments in path . segments . iter () . map (| segment | & segment . arguments) . filter (| arguments | ! arguments . is_empty ()) { arguments . hash (& mut hasher) ; is_some = true ; } is_some . then (| | hasher . finish ()) }
    };
}

hash_path_arguments!()