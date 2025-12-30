// Generated macro for impl_96 (impl)
macro_rules! Depcrate_decodeimpl_96 {
() => {
// Module: crate::decode
// Provides: {"impl_96"}
// Dependencies: {}
# [cfg (feature = "std")] impl < T : std :: io :: Read > RmpRead for T { type Error = std :: io :: Error ; # [inline] fn read_exact_buf (& mut self , buf : & mut [u8]) -> Result < () , Self :: Error > { std :: io :: Read :: read_exact (self , buf) } }
};
}
