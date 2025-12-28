macro_rules! to_abs_ws_path {
    () => {
        fn to_abs_ws_path (path : & Path) -> PathBuf { if path . is_absolute () { return path . to_owned () ; } static WORKSPACE_ROOT : OnceCell < PathBuf > = OnceCell :: new () ; WORKSPACE_ROOT . get_or_try_init (| | { if let Ok (workspace_root) = env :: var ("CARGO_WORKSPACE_DIR") { return Ok (workspace_root . into ()) ; } let my_manifest = env :: var ("CARGO_MANIFEST_DIR") ? ; let workspace_root = Path :: new (& my_manifest) . ancestors () . filter (| it | it . join ("Cargo.toml") . exists ()) . last () . unwrap () . to_path_buf () ; Ok (workspace_root) }) . unwrap_or_else (| _ : env :: VarError | { panic ! ("No CARGO_MANIFEST_DIR env var and the path is relative: {}" , path . display ()) }) . join (path) }
    };
}

to_abs_ws_path!()