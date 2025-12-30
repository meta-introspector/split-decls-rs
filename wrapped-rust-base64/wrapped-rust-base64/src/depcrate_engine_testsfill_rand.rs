// Generated macro for fill_rand (function)
macro_rules! Depcrate_engine_testsfill_rand {
() => {
// Module: crate::engine::tests
// Provides: {"fill_rand"}
// Dependencies: {}
fn fill_rand < R : rand :: Rng , D : distributions :: Distribution < usize > > (vec : & mut Vec < u8 > , rng : & mut R , length_distribution : & D ,) -> usize { let len = length_distribution . sample (rng) ; for _ in 0 .. len { vec . push (rng . gen ()) ; } len }
};
}
