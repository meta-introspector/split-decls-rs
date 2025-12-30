// Generated macro for generate_vec (function)
macro_rules! Depcrategenerate_vec {
() => {
// Module: crate
// Provides: {"generate_vec"}
// Dependencies: {}
pub fn generate_vec < T : Generate , R : rand :: Rng > (rng : & mut R , min_number : usize , max_number : usize ,) -> Vec < T > { let num : usize = rng . gen_range (min_number , max_number + 1) ; let mut res = vec ! [] ; for _ in 0 .. num { res . push (T :: generate (rng)) ; } res }
};
}
