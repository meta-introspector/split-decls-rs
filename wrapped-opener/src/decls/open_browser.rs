macro_rules! deps {
    () => {
        OpenError!();
    };
}

macro_rules! open_browser {
    () => {
        deps!();
        # [doc = " Opens a file or link with the system default program, using the `BROWSER` environment variable"] # [doc = " when set."] # [doc = ""] # [doc = " If the `BROWSER` environment variable is set, the program specified by it is used to open the"] # [doc = " path. If not, behavior is identical to [`open()`]."] pub fn open_browser < P > (path : P) -> Result < () , OpenError > where P : AsRef < OsStr > , { let mut path = path . as_ref () ; if let Ok (browser_var) = env :: var ("BROWSER") { let windows_path ; if is_wsl () && browser_var . ends_with (".exe") { if let Some (windows_path_2) = wsl_to_windows_path (path) { windows_path = windows_path_2 ; path = & windows_path ; } } ; Command :: new (& browser_var) . arg (path) . stdin (Stdio :: null ()) . stdout (Stdio :: null ()) . stderr (Stdio :: piped ()) . spawn () . map_err (| err | OpenError :: Spawn { cmds : browser_var , source : err , }) ? ; Ok (()) } else { sys :: open (path) } }
    };
}

open_browser!()