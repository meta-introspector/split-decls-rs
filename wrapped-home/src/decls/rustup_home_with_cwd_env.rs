macro_rules! deps {
    () => {
        Env!();
        OsEnv!();
    };
}

macro_rules! rustup_home_with_cwd_env {
    () => {
        deps!();
        # [doc = " Variant of `cargo_home_with_cwd` where the environment source is"] # [doc = " parameterized."] # [doc = ""] # [doc = " This is specifically to support in-process testing scenarios"] # [doc = " as environment variables and user home metadata are normally process global"] # [doc = " state. See the `OsEnv` trait."] pub fn rustup_home_with_cwd_env (env : & dyn Env , cwd : & Path) -> io :: Result < PathBuf > { match env . var_os ("RUSTUP_HOME") . filter (| h | ! h . is_empty ()) { Some (home) => { let home = PathBuf :: from (home) ; if home . is_absolute () { Ok (home) } else { Ok (cwd . join (& home)) } } _ => home_dir_with_env (env) . map (| d | d . join (".rustup")) . ok_or_else (| | io :: Error :: new (io :: ErrorKind :: Other , "could not find rustup home dir")) , } }
    };
}

rustup_home_with_cwd_env!()