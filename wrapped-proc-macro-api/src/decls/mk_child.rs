macro_rules! mk_child {
    () => {
        # [doc = " Creates and configures a new child process for the proc-macro server."] fn mk_child < 'a > (path : & AbsPath , extra_env : impl IntoIterator < Item = (impl AsRef < std :: ffi :: OsStr > , & 'a Option < impl 'a + AsRef < std :: ffi :: OsStr > >) , > ,) -> io :: Result < Child > { # [allow (clippy :: disallowed_methods)] let mut cmd = Command :: new (path) ; for env in extra_env { match env { (key , Some (val)) => cmd . env (key , val) , (key , None) => cmd . env_remove (key) , } ; } cmd . env ("RUST_ANALYZER_INTERNALS_DO_NOT_USE" , "this is unstable") . stdin (Stdio :: piped ()) . stdout (Stdio :: piped ()) . stderr (Stdio :: inherit ()) ; if cfg ! (windows) { let mut path_var = std :: ffi :: OsString :: new () ; path_var . push (path . parent () . unwrap () . parent () . unwrap ()) ; path_var . push ("\\bin;") ; path_var . push (std :: env :: var_os ("PATH") . unwrap_or_default ()) ; cmd . env ("PATH" , path_var) ; } cmd . spawn () }
    };
}

mk_child!();