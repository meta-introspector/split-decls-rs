// Generated macro for impl_186 (impl)
macro_rules! Depcrate_encode_bufferimpl_186 {
() => {
// Module: crate::encode::buffer
// Provides: {"impl_186"}
// Dependencies: {}
# [cfg (not (feature = "std"))] impl < 'a > RmpWrite for Vec < u8 > { type Error = core :: convert :: Infallible ; # [inline] fn write_u8 (& mut self , val : u8) -> Result < () , Self :: Error > { self . push (val) ; Ok (()) } # [inline] fn write_bytes (& mut self , buf : & [u8]) -> Result < () , Self :: Error > { self . extend_from_slice (buf) ; Ok (()) } }
};
}
