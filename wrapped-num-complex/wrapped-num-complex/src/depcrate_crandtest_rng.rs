// Generated macro for test_rng (function)
macro_rules! Depcrate_crandtest_rng {
() => {
// Module: crate::crand
// Provides: {"test_rng"}
// Dependencies: {}
# [cfg (test)] fn test_rng () -> impl RngCore { # [doc = " Simple `Rng` for testing without additional dependencies"] struct XorShiftStar { a : u64 , } impl RngCore for XorShiftStar { fn next_u32 (& mut self) -> u32 { self . next_u64 () as u32 } fn next_u64 (& mut self) -> u64 { self . a ^= self . a >> 12 ; self . a ^= self . a << 25 ; self . a ^= self . a >> 27 ; self . a . wrapping_mul (0x2545_F491_4F6C_DD1D) } fn fill_bytes (& mut self , dest : & mut [u8]) { for chunk in dest . chunks_mut (8) { let bytes = self . next_u64 () . to_le_bytes () ; let slice = & bytes [.. chunk . len ()] ; chunk . copy_from_slice (slice) } } fn try_fill_bytes (& mut self , dest : & mut [u8]) -> Result < () , rand :: Error > { self . fill_bytes (dest) ; Ok (()) } } XorShiftStar { a : 0x0123_4567_89AB_CDEF , } }
};
}
