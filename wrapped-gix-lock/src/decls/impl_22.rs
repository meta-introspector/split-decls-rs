macro_rules! deps {
    () => {
        Marker!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Marker { # [doc = " Return the path at which the lock file resides"] pub fn lock_path (& self) -> & Path { & self . lock_path } # [doc = " Return the path at which the locked resource resides"] pub fn resource_path (& self) -> PathBuf { strip_lock_suffix (& self . lock_path) } }
    };
}

impl_22!();