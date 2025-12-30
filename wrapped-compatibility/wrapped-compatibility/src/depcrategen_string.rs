// Generated macro for gen_string (function)
macro_rules! Depcrategen_string {
() => {
// Module: crate
// Provides: {"gen_string"}
// Dependencies: {}
pub fn gen_string (rng : & mut impl Rng) -> String { let len = rng . gen_range (0 .. 100usize) ; let mut result = String :: with_capacity (len * 4) ; for _ in 0 .. len { result . push (rng . gen_range ('\0' .. char :: MAX)) ; } result }
};
}
