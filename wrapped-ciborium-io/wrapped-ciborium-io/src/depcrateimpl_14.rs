// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
# [cfg (not (feature = "std"))] impl < R : Read + ? Sized > Read for & mut R { type Error = R :: Error ; # [inline] fn read_exact (& mut self , data : & mut [u8]) -> Result < () , Self :: Error > { (* * self) . read_exact (data) } }
};
}
