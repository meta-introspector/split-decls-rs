// Generated macro for DEFAULT_BUFFER_CAPACITY (const)
macro_rules! Depcrate_codec_framed_writeDEFAULT_BUFFER_CAPACITY {
() => {
// Module: crate::codec::framed_write
// Provides: {"DEFAULT_BUFFER_CAPACITY"}
// Dependencies: {}
# [doc = " Initialize the connection with this amount of write buffer."] # [doc = ""] # [doc = " The minimum MAX_FRAME_SIZE is 16kb, so always be able to send a HEADERS"] # [doc = " frame that big."] const DEFAULT_BUFFER_CAPACITY : usize = 16 * 1_024 ;
};
}
