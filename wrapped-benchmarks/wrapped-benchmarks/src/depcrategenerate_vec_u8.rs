// Generated macro for generate_vec_u8 (function)
macro_rules! Depcrategenerate_vec_u8 {
() => {
// Module: crate
// Provides: {"generate_vec_u8"}
// Dependencies: {}
pub fn generate_vec_u8 < R : rand :: Rng > (rng : & mut R , min_number : usize , max_number : usize) -> Vec < u8 > { let num : usize = rng . gen_range (min_number , max_number + 1) ; let mut res = vec ! [0u8 ; num] ; rng . fill_bytes (& mut res) ; res }
};
}
