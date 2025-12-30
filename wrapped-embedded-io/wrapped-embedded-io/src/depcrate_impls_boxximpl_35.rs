// Generated macro for impl_35 (impl)
macro_rules! Depcrate_impls_boxximpl_35 {
() => {
// Module: crate::impls::boxx
// Provides: {"impl_35"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < T : ? Sized + Seek > Seek for Box < T > { # [inline] fn seek (& mut self , pos : crate :: SeekFrom) -> Result < u64 , Self :: Error > { T :: seek (self , pos) } # [inline] fn rewind (& mut self) -> Result < () , Self :: Error > { T :: rewind (self) } # [inline] fn stream_position (& mut self) -> Result < u64 , Self :: Error > { T :: stream_position (self) } # [inline] fn seek_relative (& mut self , offset : i64) -> Result < () , Self :: Error > { T :: seek_relative (self , offset) } }
};
}
