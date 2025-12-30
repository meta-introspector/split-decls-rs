// Generated macro for CHAIN_THRESHOLD_WITHOUT_VECTORED_IO (const)
macro_rules! Depcrate_codec_framed_writeCHAIN_THRESHOLD_WITHOUT_VECTORED_IO {
() => {
// Module: crate::codec::framed_write
// Provides: {"CHAIN_THRESHOLD_WITHOUT_VECTORED_IO"}
// Dependencies: {}
# [doc = " Chain payloads bigger than this when vectored I/O is **not** enabled."] # [doc = " A larger value in this scenario will reduce the number of small and"] # [doc = " fragmented data being sent, and hereby improve the throughput."] const CHAIN_THRESHOLD_WITHOUT_VECTORED_IO : usize = 1024 ;
};
}
