macro_rules! deps {
    () => {
        Env!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl From < Env > for PathBuf { fn from (env : Env) -> Self { match env { Env :: Owned (os_str) => PathBuf :: from (os_str) , Env :: Arced (os_str) => PathBuf :: from (os_str . deref ()) , } } }
    };
}

impl_5!()