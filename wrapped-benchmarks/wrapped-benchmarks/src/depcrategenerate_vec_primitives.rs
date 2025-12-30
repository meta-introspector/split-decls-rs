// Generated macro for generate_vec_primitives (function)
macro_rules! Depcrategenerate_vec_primitives {
() => {
// Module: crate
// Provides: {"generate_vec_primitives"}
// Dependencies: {}
pub fn generate_vec_primitives < T , R > (rng : & mut R , min_number : usize , max_number : usize) -> Vec < T > where Standard : Distribution < T > , R : rand :: Rng , { let num : usize = rng . gen_range (min_number , max_number + 1) ; let mut res = vec ! [] ; for _ in 0 .. num { res . push (rng . gen ()) ; } res }
};
}
