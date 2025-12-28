macro_rules! deps {
    () => {
        ForUser!();
        Error!();
    };
}

macro_rules! with {
    () => {
        deps!();
        # [doc = " Expand `path` for the given `user`, which can be obtained by [`parse()`], resolving them with `home_for_user(&user)`."] # [doc = ""] # [doc = " For the common case consider using [`expand_path()]` instead."] pub fn with (user : Option < & ForUser > , path : & BStr , home_for_user : impl FnOnce (& ForUser) -> Option < PathBuf > ,) -> Result < PathBuf , Error > { fn make_relative (path : & Path) -> PathBuf { path . components () . skip (1) . collect () } let path = gix_path :: try_from_byte_slice (path) . map_err (| _ | Error :: IllformedUtf8 { path : path . to_owned () }) ? ; Ok (match user { Some (user) => home_for_user (user) . ok_or_else (| | Error :: MissingHome { user : user . to_owned () . into () , }) ? . join (make_relative (path)) , None => path . into () , }) }
    };
}

with!();