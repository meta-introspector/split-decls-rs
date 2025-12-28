macro_rules! inject_cargo_env {
    () => {
        pub (crate) fn inject_cargo_env (env : & mut Env , cargo_path : & Utf8Path) { env . set ("CARGO" , cargo_path . as_str ()) ; }
    };
}

inject_cargo_env!();