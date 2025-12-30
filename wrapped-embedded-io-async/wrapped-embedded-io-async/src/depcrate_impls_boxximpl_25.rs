// Generated macro for impl_25 (impl)
macro_rules! Depcrate_impls_boxximpl_25 {
() => {
// Module: crate::impls::boxx
// Provides: {"impl_25"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < T : ? Sized + Seek > Seek for Box < T > { # [inline] async fn seek (& mut self , pos : SeekFrom) -> Result < u64 , Self :: Error > { T :: seek (self , pos) . await } # [inline] async fn rewind (& mut self) -> Result < () , Self :: Error > { T :: rewind (self) . await } # [inline] async fn stream_position (& mut self) -> Result < u64 , Self :: Error > { T :: stream_position (self) . await } }
};
}
