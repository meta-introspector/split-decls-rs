// Generated macro for impl_67 (impl)
macro_rules! Depcrate_needleimpl_67 {
() => {
// Module: crate::needle
// Provides: {"impl_67"}
// Dependencies: {}
impl < T > Needle for Any < Vec < T > > where T : Needle , { fn check (& self , buf : & [u8] , eof : bool) -> Result < Vec < Match > , Error > { Any (self . 0 . as_slice ()) . check (buf , eof) } }
};
}
