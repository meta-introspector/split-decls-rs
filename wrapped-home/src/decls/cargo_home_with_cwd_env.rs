macro_rules! deps {
    () => {
        OsEnv!();
        Env!();
    };
}

macro_rules! cargo_home_with_cwd_env {
    () => {
        deps!();
        # [doc = " Variant of `cargo_home_with_cwd` where the environment source is"] # [doc = " parameterized."] # [doc = ""] # [doc = " This is specifically to support in-process testing scenarios"] # [doc = " as environment variables and user home metadata are normally process global"] # [doc = " state. See the `OsEnv` trait."] pub fn cargo_home_with_cwd_env (env : & dyn Env , cwd : & Path) -> io :: Result < PathBuf > { match env . var_os ("CARGO_HOME") . filter (| h | ! h . is_empty ()) { Some (home) => { let home = PathBuf :: from (home) ; if home . is_absolute () { Ok (home) } else { Ok (cwd . join (& home)) } } _ => home_dir_with_env (env) . map (| p | p . join (".cargo")) . ok_or_else (| | io :: Error :: new (io :: ErrorKind :: Other , "could not find cargo home dir")) , } }
    };
}

cargo_home_with_cwd_env!()