macro_rules! deps {
    () => {
        OpenError!();
    };
}

macro_rules! non_wsl_open {
    () => {
        deps!();
        fn non_wsl_open (path : & OsStr) -> Result < () , OpenError > { if open_with_system_xdg_open (path) . is_err () { open_with_internal_xdg_open (path) ? ; } Ok (()) }
    };
}

non_wsl_open!()