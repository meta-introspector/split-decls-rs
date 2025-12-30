// Generated macro for impl_120 (impl)
macro_rules! Depcrate_hazardous_aead_streamingimpl_120 {
() => {
// Module: crate::hazardous::aead::streaming
// Provides: {"impl_120"}
// Dependencies: {}
impl TryFrom < u8 > for StreamTag { type Error = UnknownCryptoError ; fn try_from (byte : u8) -> Result < Self , Self :: Error > { match byte { 0b0000_0000 => Ok (Self :: Message) , 0b0000_0001 => Ok (Self :: Push) , 0b0000_0010 => Ok (Self :: Rekey) , 0b0000_0011 => Ok (Self :: Finish) , _ => Err (UnknownCryptoError) , } } }
};
}
