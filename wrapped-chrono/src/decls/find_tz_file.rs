macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! find_tz_file {
    () => {
        deps!();
        # [doc = " Open the TZif file corresponding to a TZ string"] fn find_tz_file (path : impl AsRef < Path >) -> Result < File , Error > { # [cfg (not (unix))] return Ok (File :: open (path) ?) ; # [cfg (unix)] { let path = path . as_ref () ; if path . is_absolute () { return Ok (File :: open (path) ?) ; } for folder in & ZONE_INFO_DIRECTORIES { if let Ok (file) = File :: open (PathBuf :: from (folder) . join (path)) { return Ok (file) ; } } Err (Error :: Io (io :: ErrorKind :: NotFound . into ())) } }
    };
}

find_tz_file!()