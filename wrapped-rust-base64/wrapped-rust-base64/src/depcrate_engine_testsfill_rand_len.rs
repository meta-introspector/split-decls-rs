// Generated macro for fill_rand_len (function)
macro_rules! Depcrate_engine_testsfill_rand_len {
() => {
// Module: crate::engine::tests
// Provides: {"fill_rand_len"}
// Dependencies: {}
fn fill_rand_len < R : rand :: Rng > (vec : & mut Vec < u8 > , rng : & mut R , len : usize) { for _ in 0 .. len { vec . push (rng . gen ()) ; } }
};
}
