// Generated macro for impl_9 (impl)
macro_rules! Depcrate_impls_blanketimpl_9 {
() => {
// Module: crate::impls::blanket
// Provides: {"impl_9"}
// Dependencies: {}
impl < T : ? Sized + Seek > Seek for & mut T { # [inline] async fn seek (& mut self , pos : SeekFrom) -> Result < u64 , Self :: Error > { T :: seek (self , pos) . await } # [inline] async fn rewind (& mut self) -> Result < () , Self :: Error > { T :: rewind (self) . await } # [inline] async fn stream_position (& mut self) -> Result < u64 , Self :: Error > { T :: stream_position (self) . await } }
};
}
