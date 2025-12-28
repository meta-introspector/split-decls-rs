macro_rules! deps {
    () => {
        Env!();
        OsEnv!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Env for OsEnv { fn home_dir (& self) -> Option < PathBuf > { crate :: home_dir_inner () } fn current_dir (& self) -> io :: Result < PathBuf > { std :: env :: current_dir () } fn var_os (& self , key : & str) -> Option < OsString > { std :: env :: var_os (key) } }
    };
}

impl_2!();