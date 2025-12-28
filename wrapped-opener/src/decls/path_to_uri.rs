macro_rules! deps {
    () => {
        OpenError!();
    };
}

macro_rules! path_to_uri {
    () => {
        deps!();
        fn path_to_uri (path : & Path) -> Result < Url , OpenError > { let path = path . canonicalize () . map_err (OpenError :: Io) ? ; Url :: from_file_path (path) . map_err (| _ | uri_to_open_error ()) }
    };
}

path_to_uri!()