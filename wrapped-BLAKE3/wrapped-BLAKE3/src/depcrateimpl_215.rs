// Generated macro for impl_215 (impl)
macro_rules! Depcrateimpl_215 {
() => {
// Module: crate
// Provides: {"impl_215"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl Zeroize for ChunkState { fn zeroize (& mut self) { let Self { cv , chunk_counter , buf , buf_len , blocks_compressed , flags , platform : _ , } = self ; cv . zeroize () ; chunk_counter . zeroize () ; buf . zeroize () ; buf_len . zeroize () ; blocks_compressed . zeroize () ; flags . zeroize () ; } }
};
}
