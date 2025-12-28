macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! relative_path_from_value_and_path {
    () => {
        deps!();
        fn relative_path_from_value_and_path < 'a > (path_bstr : & 'a BStr , path : & Path) -> Result < & 'a RelativePath , Error > { if path . is_absolute () { return Err (Error :: IsAbsolute) ; } let options = Options :: default () ; for component in path . components () { let component = os_str_into_bstr (component . as_os_str ()) ? ; gix_validate :: path :: component (component , None , options) ? ; } RelativePath :: new_unchecked (BStr :: new (path_bstr . as_bytes ())) }
    };
}

relative_path_from_value_and_path!();