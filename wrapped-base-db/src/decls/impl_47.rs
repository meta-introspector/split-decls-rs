macro_rules! deps {
    () => {
        BuiltDependency!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl BuiltDependency { # [doc = " Whether this dependency is to be added to the depending crate's extern prelude."] pub fn is_prelude (& self) -> bool { self . prelude } # [doc = " Whether this dependency is a sysroot injected one."] pub fn is_sysroot (& self) -> bool { self . sysroot } }
    };
}

impl_47!();