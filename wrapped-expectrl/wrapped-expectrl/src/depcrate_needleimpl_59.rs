// Generated macro for impl_59 (impl)
macro_rules! Depcrate_needleimpl_59 {
() => {
// Module: crate::needle
// Provides: {"impl_59"}
// Dependencies: {}
impl Needle for str { fn check (& self , buf : & [u8] , eof : bool) -> Result < Vec < Match > , Error > { self . as_bytes () . check (buf , eof) } }
};
}
