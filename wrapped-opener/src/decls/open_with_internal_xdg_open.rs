macro_rules! deps {
    () => {
        OpenError!();
    };
}

macro_rules! open_with_internal_xdg_open {
    () => {
        deps!();
        fn open_with_internal_xdg_open (path : & OsStr) -> Result < Child , OpenError > { let mut sh = Command :: new ("sh") . arg ("-s") . arg (path) . stdin (Stdio :: piped ()) . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . spawn () . map_err (| err | OpenError :: Spawn { cmds : "sh" . into () , source : err , }) ? ; sh . stdin . as_mut () . unwrap () . write_all (XDG_OPEN_SCRIPT) . map_err (OpenError :: Io) ? ; Ok (sh) }
    };
}

open_with_internal_xdg_open!()