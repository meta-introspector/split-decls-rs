// Generated macro for impl_347 (impl)
macro_rules! Depcrateimpl_347 {
() => {
// Module: crate
// Provides: {"impl_347"}
// Dependencies: {}
impl NixPath for CStr { fn is_empty (& self) -> bool { self . to_bytes () . is_empty () } fn len (& self) -> usize { self . to_bytes () . len () } fn with_nix_path < T , F > (& self , f : F) -> Result < T > where F : FnOnce (& CStr) -> T , { Ok (f (self)) } }
};
}
