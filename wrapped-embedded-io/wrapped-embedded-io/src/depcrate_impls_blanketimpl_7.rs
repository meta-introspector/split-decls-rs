// Generated macro for impl_7 (impl)
macro_rules! Depcrate_impls_blanketimpl_7 {
() => {
// Module: crate::impls::blanket
// Provides: {"impl_7"}
// Dependencies: {}
impl < T : ? Sized + Read > Read for & mut T { # [inline] fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { T :: read (self , buf) } # [inline] fn read_exact (& mut self , buf : & mut [u8]) -> Result < () , ReadExactError < Self :: Error > > { T :: read_exact (self , buf) } }
};
}
