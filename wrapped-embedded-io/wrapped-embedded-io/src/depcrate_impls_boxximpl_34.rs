// Generated macro for impl_34 (impl)
macro_rules! Depcrate_impls_boxximpl_34 {
() => {
// Module: crate::impls::boxx
// Provides: {"impl_34"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl < T : ? Sized + Write > Write for Box < T > { # [inline] fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { T :: write (self , buf) } # [inline] fn write_all (& mut self , buf : & [u8]) -> Result < () , Self :: Error > { T :: write_all (self , buf) } # [inline] fn write_fmt (& mut self , fmt : core :: fmt :: Arguments < '_ > ,) -> Result < () , crate :: WriteFmtError < Self :: Error > > { T :: write_fmt (self , fmt) } # [inline] fn flush (& mut self) -> Result < () , Self :: Error > { T :: flush (self) } }
};
}
