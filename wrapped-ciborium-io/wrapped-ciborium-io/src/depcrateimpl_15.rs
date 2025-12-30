// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
# [cfg (not (feature = "std"))] impl < W : Write + ? Sized > Write for & mut W { type Error = W :: Error ; # [inline] fn write_all (& mut self , data : & [u8]) -> Result < () , Self :: Error > { (* * self) . write_all (data) } # [inline] fn flush (& mut self) -> Result < () , Self :: Error > { (* * self) . flush () } }
};
}
