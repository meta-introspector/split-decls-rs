macro_rules! deps {
    () => {
        OpenError!();
    };
}

macro_rules! wsl_open {
    () => {
        deps!();
        fn wsl_open (path : & OsStr) -> Result < () , OpenError > { let result = open_with_wslview (path) ; if let Ok (mut child) = result { return crate :: wait_child (& mut child , "wslview") ; } open_with_system_xdg_open (path) . map_err (| err | OpenError :: Spawn { cmds : "wslview, xdg-open" . into () , source : err , }) ? ; Ok (()) }
    };
}

wsl_open!();