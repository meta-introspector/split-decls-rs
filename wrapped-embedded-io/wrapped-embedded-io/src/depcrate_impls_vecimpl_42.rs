// Generated macro for impl_42 (impl)
macro_rules! Depcrate_impls_vecimpl_42 {
() => {
// Module: crate::impls::vec
// Provides: {"impl_42"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl Write for Vec < u8 > { # [inline] fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { self . extend_from_slice (buf) ; Ok (buf . len ()) } # [inline] fn write_all (& mut self , buf : & [u8]) -> Result < () , Self :: Error > { self . write (buf) ? ; Ok (()) } # [inline] fn flush (& mut self) -> Result < () , Self :: Error > { Ok (()) } }
};
}
