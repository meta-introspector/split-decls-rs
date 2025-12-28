macro_rules! deps {
    () => {
        NixPath!();
        Result!();
    };
}

macro_rules! impl_278 {
    () => {
        deps!();
        impl NixPath for OsStr { fn is_empty (& self) -> bool { self . as_bytes () . is_empty () } fn len (& self) -> usize { self . as_bytes () . len () } fn with_nix_path < T , F > (& self , f : F) -> Result < T > where F : FnOnce (& CStr) -> T , { self . as_bytes () . with_nix_path (f) } }
    };
}

impl_278!()