// Generated macro for impl_6 (impl)
macro_rules! Depcrate_impls_blanketimpl_6 {
() => {
// Module: crate::impls::blanket
// Provides: {"impl_6"}
// Dependencies: {}
impl < T : ? Sized + Read > Read for & mut T { # [inline] async fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { T :: read (self , buf) . await } # [inline] async fn read_exact (& mut self , buf : & mut [u8]) -> Result < () , ReadExactError < Self :: Error > > { T :: read_exact (self , buf) . await } }
};
}
