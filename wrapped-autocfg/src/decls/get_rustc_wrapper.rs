macro_rules! get_rustc_wrapper {
    () => {
        fn get_rustc_wrapper (workspace : bool) -> Option < PathBuf > { if workspace && env :: var_os ("CARGO_ENCODED_RUSTFLAGS") . is_none () { return None ; } let name = if workspace { "RUSTC_WORKSPACE_WRAPPER" } else { "RUSTC_WRAPPER" } ; if let Some (wrapper) = env :: var_os (name) { if wrapper != OsString :: new () { return Some (wrapper . into ()) ; } } None }
    };
}

get_rustc_wrapper!();