// Generated macro for impl_37 (impl)
macro_rules! Depcrate_keysimpl_37 {
() => {
// Module: crate::keys
// Provides: {"impl_37"}
// Dependencies: {}
impl AsName for [u8] { fn name (& self) -> & Name { assert ! (! self . is_empty () , "cannot create Name from empty byte-string") ; assert_eq ! (* self . last () . unwrap () , b'\0' , "cannot create Name from non-null-terminated byte-string \"{}\"" , str :: from_utf8 (self) . unwrap ()) ; unsafe { & * (self as * const Self as * const Name) } } }
};
}
