macro_rules! path_name_to_string {
    () => {
        # [doc = " Return a \"error message-able\" ident for the last segment of the `Path`"] fn path_name_to_string (path : & Path < '_ >) -> String { path . segments . last () . unwrap () . ident . to_string () }
    };
}

path_name_to_string!();