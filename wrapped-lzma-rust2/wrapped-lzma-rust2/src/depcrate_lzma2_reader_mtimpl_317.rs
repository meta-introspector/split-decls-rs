// Generated macro for impl_317 (impl)
macro_rules! Depcrate_lzma2_reader_mtimpl_317 {
() => {
// Module: crate::lzma2_reader_mt
// Provides: {"impl_317"}
// Dependencies: {}
impl < R : Read > Drop for Lzma2ReaderMt < R > { fn drop (& mut self) { self . shutdown_flag . store (true , Ordering :: Release) ; self . work_queue . close () ; } }
};
}
