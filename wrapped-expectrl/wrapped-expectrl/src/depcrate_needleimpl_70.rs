// Generated macro for impl_70 (impl)
macro_rules! Depcrate_needleimpl_70 {
() => {
// Module: crate::needle
// Provides: {"impl_70"}
// Dependencies: {}
impl < T : Needle > Needle for & T { fn check (& self , buf : & [u8] , eof : bool) -> Result < Vec < Match > , Error > { T :: check (self , buf , eof) } }
};
}
