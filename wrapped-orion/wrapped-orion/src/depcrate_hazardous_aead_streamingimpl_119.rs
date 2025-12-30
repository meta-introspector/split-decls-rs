// Generated macro for impl_119 (impl)
macro_rules! Depcrate_hazardous_aead_streamingimpl_119 {
() => {
// Module: crate::hazardous::aead::streaming
// Provides: {"impl_119"}
// Dependencies: {}
impl StreamTag { # [inline] # [doc = " Return the tag as a byte."] pub fn as_byte (& self) -> u8 { match * self { StreamTag :: Message => 0b0000_0000 , StreamTag :: Push => 0b0000_0001 , StreamTag :: Rekey => 0b0000_0010 , StreamTag :: Finish => 0b0000_0011 , } } }
};
}
