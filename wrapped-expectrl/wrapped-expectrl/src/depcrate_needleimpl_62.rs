// Generated macro for impl_62 (impl)
macro_rules! Depcrate_needleimpl_62 {
() => {
// Module: crate::needle
// Provides: {"impl_62"}
// Dependencies: {}
impl Needle for u8 { fn check (& self , buf : & [u8] , eof : bool) -> Result < Vec < Match > , Error > { ([* self] [..]) . check (buf , eof) } }
};
}
