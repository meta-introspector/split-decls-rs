// Generated macro for impl_24 (impl)
macro_rules! Depcrate_impls_boxximpl_24 {
() => {
// Module: crate::impls::boxx
// Provides: {"impl_24"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < T : ? Sized + Write > Write for Box < T > { # [inline] async fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { T :: write (self , buf) . await } # [inline] async fn write_all (& mut self , buf : & [u8]) -> Result < () , Self :: Error > { T :: write_all (self , buf) . await } # [inline] async fn flush (& mut self) -> Result < () , Self :: Error > { T :: flush (self) . await } }
};
}
