macro_rules! deps {
    () => {
        StdEnvGetter!();
        EnvGetter!();
        Env!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl EnvGetter for StdEnvGetter { # [allow (clippy :: disallowed_methods)] fn get_env (& self , name : & 'static str) -> Option < Env > { env :: var_os (name) . map (Env :: Owned) } }
    };
}

impl_8!();