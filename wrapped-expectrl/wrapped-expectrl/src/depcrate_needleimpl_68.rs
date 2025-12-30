// Generated macro for impl_68 (impl)
macro_rules! Depcrate_needleimpl_68 {
() => {
// Module: crate::needle
// Provides: {"impl_68"}
// Dependencies: {}
impl < T , const N : usize > Needle for Any < [T ; N] > where T : Needle , { fn check (& self , buf : & [u8] , eof : bool) -> Result < Vec < Match > , Error > { Any (& self . 0 [..]) . check (buf , eof) } }
};
}
