// Generated macro for CHAIN_THRESHOLD (const)
macro_rules! Depcrate_codec_framed_writeCHAIN_THRESHOLD {
() => {
// Module: crate::codec::framed_write
// Provides: {"CHAIN_THRESHOLD"}
// Dependencies: {}
# [doc = " Chain payloads bigger than this when vectored I/O is enabled. The remote"] # [doc = " will never advertise a max frame size less than this (well, the spec says"] # [doc = " the max frame size can't be less than 16kb, so not even close)."] const CHAIN_THRESHOLD : usize = 256 ;
};
}
