// Generated macro for impl_350 (impl)
macro_rules! Depcrateimpl_350 {
() => {
// Module: crate
// Provides: {"impl_350"}
// Dependencies: {}
impl NixPath for Path { fn is_empty (& self) -> bool { NixPath :: is_empty (self . as_os_str ()) } fn len (& self) -> usize { NixPath :: len (self . as_os_str ()) } fn with_nix_path < T , F > (& self , f : F) -> Result < T > where F : FnOnce (& CStr) -> T , { self . as_os_str () . with_nix_path (f) } }
};
}
