// Generated macro for impl_202 (impl)
macro_rules! Depcrate_encodeimpl_202 {
() => {
// Module: crate::encode
// Provides: {"impl_202"}
// Dependencies: {}
# [cfg (feature = "std")] impl < T : std :: io :: Write > RmpWrite for T { type Error = std :: io :: Error ; # [inline] fn write_bytes (& mut self , buf : & [u8]) -> Result < () , Self :: Error > { self . write_all (buf) } }
};
}
