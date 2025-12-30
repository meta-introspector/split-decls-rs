// Generated macro for impl_185 (impl)
macro_rules! Depcrate_encode_bufferimpl_185 {
() => {
// Module: crate::encode::buffer
// Provides: {"impl_185"}
// Dependencies: {}
impl RmpWrite for ByteBuf { type Error = core :: convert :: Infallible ; # [inline] fn write_u8 (& mut self , val : u8) -> Result < () , Self :: Error > { self . bytes . push (val) ; Ok (()) } # [inline] fn write_bytes (& mut self , buf : & [u8]) -> Result < () , Self :: Error > { self . bytes . extend_from_slice (buf) ; Ok (()) } }
};
}
