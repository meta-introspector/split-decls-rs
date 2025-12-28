macro_rules! deps {
    () => {
        CompletionCandidate!();
    };
}

macro_rules! complete_path {
    () => {
        deps!();
        pub (crate) fn complete_path (value_os : & OsStr , current_dir : Option < & std :: path :: Path > , is_wanted : & dyn Fn (& std :: path :: Path) -> bool ,) -> Vec < CompletionCandidate > { let mut completions = Vec :: new () ; let mut potential = Vec :: new () ; let value_path = std :: path :: Path :: new (value_os) ; let (prefix , current) = split_file_name (value_path) ; let current = current . to_string_lossy () ; let search_root = if prefix . is_absolute () { prefix . to_owned () } else if prefix . iter () . next () == Some (OsStr :: new ("~")) { let prefix = prefix . strip_prefix ("~") . unwrap_or (prefix) ; let home_dir = match std :: env :: home_dir () { Some (home_dir) => home_dir , None => { return completions ; } } ; home_dir . join (prefix) } else { let current_dir = match current_dir { Some (current_dir) => current_dir , None => { return completions ; } } ; current_dir . join (prefix) } ; debug ! ("complete_path: search_root={search_root:?}, prefix={prefix:?}") ; if value_os . is_empty () && is_wanted (& search_root) { completions . push ("." . into ()) ; } for entry in std :: fs :: read_dir (& search_root) . ok () . into_iter () . flatten () . filter_map (Result :: ok) { let raw_file_name = entry . file_name () ; if ! raw_file_name . starts_with (& current) { continue ; } if entry . metadata () . map (| m | m . is_dir ()) . unwrap_or (false) { let mut suggestion = prefix . join (& raw_file_name) ; suggestion . push ("") ; let candidate = CompletionCandidate :: new (suggestion . as_os_str () . to_owned ()) . hide (is_hidden (& raw_file_name)) ; if is_wanted (& entry . path ()) { completions . push (candidate) ; } else { potential . push (candidate) ; } } else { if is_wanted (& entry . path ()) { let suggestion = prefix . join (& raw_file_name) ; let candidate = CompletionCandidate :: new (suggestion . as_os_str () . to_owned ()) . hide (is_hidden (& raw_file_name)) ; completions . push (candidate) ; } } } completions . sort () ; potential . sort () ; completions . extend (potential) ; completions }
    };
}

complete_path!();