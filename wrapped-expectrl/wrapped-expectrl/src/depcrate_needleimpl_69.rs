// Generated macro for impl_69 (impl)
macro_rules! Depcrate_needleimpl_69 {
() => {
// Module: crate::needle
// Provides: {"impl_69"}
// Dependencies: {}
impl < T , const N : usize > Needle for Any < & '_ [T ; N] > where T : Needle , { fn check (& self , buf : & [u8] , eof : bool) -> Result < Vec < Match > , Error > { Any (& self . 0 [..]) . check (buf , eof) } }
};
}
