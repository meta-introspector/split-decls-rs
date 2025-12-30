// Generated macro for impl_354 (impl)
macro_rules! Depcrate_nostd_ioimpl_354 {
() => {
// Module: crate::nostd_io
// Provides: {"impl_354"}
// Dependencies: {}
impl < W : Write + ? Sized > Write for & mut W { # [inline] fn write (& mut self , buf : & [u8]) -> Result < usize > { (* * self) . write (buf) } # [inline] fn flush (& mut self) -> Result < () > { (* * self) . flush () } # [inline] fn write_all (& mut self , buf : & [u8]) -> Result < () > { (* * self) . write_all (buf) } # [inline] fn write_fmt (& mut self , fmt : fmt :: Arguments < '_ >) -> Result < () > { (* * self) . write_fmt (fmt) } }
};
}
