// Generated macro for impl_63 (impl)
macro_rules! Depcrate_needleimpl_63 {
() => {
// Module: crate::needle
// Provides: {"impl_63"}
// Dependencies: {}
impl Needle for char { fn check (& self , buf : & [u8] , eof : bool) -> Result < Vec < Match > , Error > { char :: to_string (self) . check (buf , eof) } }
};
}
