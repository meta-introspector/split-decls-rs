// Generated macro for impl_71 (impl)
macro_rules! Depcrate_needleimpl_71 {
() => {
// Module: crate::needle
// Provides: {"impl_71"}
// Dependencies: {}
impl Needle for Box < dyn Needle + '_ > { fn check (& self , buf : & [u8] , eof : bool) -> Result < Vec < Match > , Error > { self . as_ref () . check (buf , eof) } }
};
}
