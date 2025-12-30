// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
# [cfg (feature = "std")] impl < T : std :: io :: Read > Read for T { type Error = std :: io :: Error ; # [inline] fn read_exact (& mut self , data : & mut [u8]) -> Result < () , Self :: Error > { self . read_exact (data) } }
};
}
