// Generated macro for ResultUnit (type)
macro_rules! Depcrate_lzma2_reader_mtResultUnit {
() => {
// Module: crate::lzma2_reader_mt
// Provides: {"ResultUnit"}
// Dependencies: {}
# [doc = " A result unit from a worker thread."] # [doc = " Contains the sequence number and the decompressed data."] type ResultUnit = (u64 , Vec < u8 >) ;
};
}
