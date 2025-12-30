// Generated macro for impl_1228 (impl)
macro_rules! Depcrate_test_runner_rngimpl_1228 {
() => {
// Module: crate::test_runner::rng
// Provides: {"impl_1228"}
// Dependencies: {}
impl RngCore for TestRng { fn next_u32 (& mut self) -> u32 { match & mut self . rng { TestRngImpl :: XorShift (rng) => rng . next_u32 () , TestRngImpl :: ChaCha (rng) => rng . next_u32 () , TestRngImpl :: PassThrough { .. } => { let mut buf = [0 ; 4] ; self . fill_bytes (& mut buf [..]) ; u32 :: from_le_bytes (buf) } TestRngImpl :: Recorder { rng , record } => { let read = rng . next_u32 () ; record . extend_from_slice (& read . to_le_bytes ()) ; read } } } fn next_u64 (& mut self) -> u64 { match & mut self . rng { TestRngImpl :: XorShift (rng) => rng . next_u64 () , TestRngImpl :: ChaCha (rng) => rng . next_u64 () , TestRngImpl :: PassThrough { .. } => { let mut buf = [0 ; 8] ; self . fill_bytes (& mut buf [..]) ; u64 :: from_le_bytes (buf) } TestRngImpl :: Recorder { rng , record } => { let read = rng . next_u64 () ; record . extend_from_slice (& read . to_le_bytes ()) ; read } } } fn fill_bytes (& mut self , dest : & mut [u8]) { match & mut self . rng { TestRngImpl :: XorShift (rng) => rng . fill_bytes (dest) , TestRngImpl :: ChaCha (rng) => rng . fill_bytes (dest) , TestRngImpl :: PassThrough { off , end , data } => { let bytes_to_copy = dest . len () . min (* end - * off) ; dest [.. bytes_to_copy] . copy_from_slice (& data [* off .. * off + bytes_to_copy]) ; * off += bytes_to_copy ; for i in bytes_to_copy .. dest . len () { dest [i] = 0 ; } } TestRngImpl :: Recorder { rng , record } => { rng . fill_bytes (dest) ; record . extend_from_slice (dest) ; } } } }
};
}
