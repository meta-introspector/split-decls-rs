// Generated macro for impl_32 (impl)
macro_rules! Depcrate_impls_boxximpl_32 {
() => {
// Module: crate::impls::boxx
// Provides: {"impl_32"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < T : ? Sized + Read > Read for Box < T > { # [inline] fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { T :: read (self , buf) } # [inline] fn read_exact (& mut self , buf : & mut [u8]) -> Result < () , crate :: ReadExactError < Self :: Error > > { T :: read_exact (self , buf) } }
};
}
