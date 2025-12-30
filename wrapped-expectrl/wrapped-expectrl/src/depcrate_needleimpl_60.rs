// Generated macro for impl_60 (impl)
macro_rules! Depcrate_needleimpl_60 {
() => {
// Module: crate::needle
// Provides: {"impl_60"}
// Dependencies: {}
impl Needle for & str { fn check (& self , buf : & [u8] , eof : bool) -> Result < Vec < Match > , Error > { self . as_bytes () . check (buf , eof) } }
};
}
