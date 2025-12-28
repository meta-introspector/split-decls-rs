macro_rules! deps {
    () => {
        Result!();
        NixPath!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl NixPath for Path { fn is_empty (& self) -> bool { NixPath :: is_empty (self . as_os_str ()) } fn len (& self) -> usize { NixPath :: len (self . as_os_str ()) } fn with_nix_path < T , F > (& self , f : F) -> Result < T > where F : FnOnce (& CStr) -> T , { self . as_os_str () . with_nix_path (f) } }
    };
}

impl_282!();