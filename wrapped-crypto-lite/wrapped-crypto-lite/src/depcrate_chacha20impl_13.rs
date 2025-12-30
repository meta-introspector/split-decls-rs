// Generated macro for impl_13 (impl)
macro_rules! Depcrate_chacha20impl_13 {
() => {
// Module: crate::chacha20
// Provides: {"impl_13"}
// Dependencies: {}
impl Drop for ChaCha20 { fn drop (& mut self) { self . state = [0 ; STATE_WORDS] ; self . buffer = [0 ; BUFFER_SIZE] ; self . buffer_pos = 0 ; core :: sync :: atomic :: fence (core :: sync :: atomic :: Ordering :: Release) ; } }
};
}
