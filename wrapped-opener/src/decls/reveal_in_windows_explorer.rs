macro_rules! deps {
    () => {
        OpenError!();
    };
}

macro_rules! reveal_in_windows_explorer {
    () => {
        deps!();
        # [cfg (all (feature = "reveal" , target_os = "linux"))] fn reveal_in_windows_explorer (path : & std :: path :: Path) -> Result < () , OpenError > { let converted_path = crate :: wsl_to_windows_path (path . as_os_str ()) ; let converted_path = converted_path . as_deref () ; let path = match converted_path { None => path , Some (x) => std :: path :: Path :: new (x) , } ; Command :: new ("explorer.exe") . arg ("/select,") . arg (path) . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . spawn () . map_err (| err | OpenError :: Spawn { cmds : "explorer.exe" . into () , source : err , }) ? ; Ok (()) }
    };
}

reveal_in_windows_explorer!();