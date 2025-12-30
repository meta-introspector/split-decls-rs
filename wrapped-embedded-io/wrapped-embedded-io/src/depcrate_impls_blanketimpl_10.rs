// Generated macro for impl_10 (impl)
macro_rules! Depcrate_impls_blanketimpl_10 {
() => {
// Module: crate::impls::blanket
// Provides: {"impl_10"}
// Dependencies: {}
impl < T : ? Sized + Seek > Seek for & mut T { # [inline] fn seek (& mut self , pos : SeekFrom) -> Result < u64 , Self :: Error > { T :: seek (self , pos) } # [inline] fn rewind (& mut self) -> Result < () , Self :: Error > { T :: rewind (self) } # [inline] fn stream_position (& mut self) -> Result < u64 , Self :: Error > { T :: stream_position (self) } # [inline] fn seek_relative (& mut self , offset : i64) -> Result < () , Self :: Error > { T :: seek_relative (self , offset) } }
};
}
