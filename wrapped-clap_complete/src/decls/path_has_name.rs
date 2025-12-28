macro_rules! path_has_name {
    () => {
        fn path_has_name (path : & std :: path :: Path) -> bool { let path_bytes = path . as_os_str () . as_encoded_bytes () ; let Some (trailing) = path_bytes . last () else { return false ; } ; let trailing = * trailing as char ; ! std :: path :: is_separator (trailing) && path . file_name () . is_some () }
    };
}

path_has_name!();