macro_rules! deps {
    () => {
        Environment!();
    };
}

macro_rules! home_dir {
    () => {
        deps!();
        pub (crate) fn home_dir (environment : crate :: open :: permissions :: Environment) -> Option < PathBuf > { gix_path :: env :: home_dir () . and_then (| path | environment . home . check_opt (path)) }
    };
}

home_dir!();