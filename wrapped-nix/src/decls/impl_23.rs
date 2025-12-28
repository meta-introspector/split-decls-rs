macro_rules! deps {
    () => {
        NixPath!();
        Result!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl NixPath for str { fn is_empty (& self) -> bool { NixPath :: is_empty (OsStr :: new (self)) } fn len (& self) -> usize { NixPath :: len (OsStr :: new (self)) } fn with_nix_path < T , F > (& self , f : F) -> Result < T > where F : FnOnce (& CStr) -> T , { OsStr :: new (self) . with_nix_path (f) } }
    };
}

impl_23!()