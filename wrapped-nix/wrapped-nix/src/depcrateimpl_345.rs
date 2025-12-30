// Generated macro for impl_345 (impl)
macro_rules! Depcrateimpl_345 {
() => {
// Module: crate
// Provides: {"impl_345"}
// Dependencies: {}
impl NixPath for str { fn is_empty (& self) -> bool { NixPath :: is_empty (OsStr :: new (self)) } fn len (& self) -> usize { NixPath :: len (OsStr :: new (self)) } fn with_nix_path < T , F > (& self , f : F) -> Result < T > where F : FnOnce (& CStr) -> T , { OsStr :: new (self) . with_nix_path (f) } }
};
}
