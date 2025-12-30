// Generated macro for impl_8 (impl)
macro_rules! Depcrate_impls_blanketimpl_8 {
() => {
// Module: crate::impls::blanket
// Provides: {"impl_8"}
// Dependencies: {}
impl < T : ? Sized + Write > Write for & mut T { # [inline] async fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { T :: write (self , buf) . await } # [inline] async fn flush (& mut self) -> Result < () , Self :: Error > { T :: flush (self) . await } # [inline] async fn write_all (& mut self , buf : & [u8]) -> Result < () , Self :: Error > { T :: write_all (self , buf) . await } }
};
}
