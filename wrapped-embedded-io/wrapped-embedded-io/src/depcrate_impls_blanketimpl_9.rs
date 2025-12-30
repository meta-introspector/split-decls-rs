// Generated macro for impl_9 (impl)
macro_rules! Depcrate_impls_blanketimpl_9 {
() => {
// Module: crate::impls::blanket
// Provides: {"impl_9"}
// Dependencies: {}
impl < T : ? Sized + Write > Write for & mut T { # [inline] fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { T :: write (self , buf) } # [inline] fn flush (& mut self) -> Result < () , Self :: Error > { T :: flush (self) } # [inline] fn write_all (& mut self , buf : & [u8]) -> Result < () , Self :: Error > { T :: write_all (self , buf) } # [inline] fn write_fmt (& mut self , fmt : fmt :: Arguments < '_ >) -> Result < () , WriteFmtError < Self :: Error > > { T :: write_fmt (self , fmt) } }
};
}
