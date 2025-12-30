// Generated macro for Lzma2WriterMt (struct)
macro_rules! Depcrate_enc_lzma2_writer_mtLzma2WriterMt {
() => {
// Module: crate::enc::lzma2_writer_mt
// Provides: {"Lzma2WriterMt"}
// Dependencies: {}
# [doc = " A multi-threaded LZMA2 compressor."] pub struct Lzma2WriterMt < W : Write > { inner : W , options : Lzma2Options , chunk_size : usize , current_work_unit : Vec < u8 > , work_pool : WorkPool < WorkUnit , Vec < u8 > > , }
};
}
