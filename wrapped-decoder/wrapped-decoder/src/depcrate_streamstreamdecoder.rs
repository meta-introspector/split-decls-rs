// Generated macro for StreamDecoder (trait)
macro_rules! Depcrate_streamStreamDecoder {
() => {
// Module: crate::stream
// Provides: {"StreamDecoder"}
// Dependencies: {}
pub trait StreamDecoder { # [doc = " Push received data to the decoder. The decoder stores it"] # [doc = " internally, and makes decoded frames available through [`decode`](StreamDecoder::decode)."] fn received (& mut self , data : & [u8]) ; fn decode (& mut self) -> Result < Frame < '_ > , DecodeError > ; }
};
}
