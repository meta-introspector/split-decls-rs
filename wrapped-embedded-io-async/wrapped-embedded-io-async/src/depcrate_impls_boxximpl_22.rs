// Generated macro for impl_22 (impl)
macro_rules! Depcrate_impls_boxximpl_22 {
() => {
// Module: crate::impls::boxx
// Provides: {"impl_22"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < T : ? Sized + Read > Read for Box < T > { # [inline] async fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { T :: read (self , buf) . await } # [inline] async fn read_exact (& mut self , buf : & mut [u8] ,) -> Result < () , crate :: ReadExactError < Self :: Error > > { T :: read_exact (self , buf) . await } }
};
}
