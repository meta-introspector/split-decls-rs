// Generated macro for impl_61 (impl)
macro_rules! Depcrate_needleimpl_61 {
() => {
// Module: crate::needle
// Provides: {"impl_61"}
// Dependencies: {}
impl Needle for String { fn check (& self , buf : & [u8] , eof : bool) -> Result < Vec < Match > , Error > { self . as_bytes () . check (buf , eof) } }
};
}
