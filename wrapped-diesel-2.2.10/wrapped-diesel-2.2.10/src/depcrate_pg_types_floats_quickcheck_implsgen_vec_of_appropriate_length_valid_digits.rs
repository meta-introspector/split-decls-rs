// Generated macro for gen_vec_of_appropriate_length_valid_digits (function)
macro_rules! Depcrate_pg_types_floats_quickcheck_implsgen_vec_of_appropriate_length_valid_digits {
() => {
// Module: crate::pg::types::floats::quickcheck_impls
// Provides: {"gen_vec_of_appropriate_length_valid_digits"}
// Dependencies: {}
fn gen_vec_of_appropriate_length_valid_digits (g : & mut Gen , weight : u16 , scale : u16) -> Vec < i16 > { let max_digits = :: std :: cmp :: min (weight , scale) ; let mut digits = Vec :: < Digit > :: arbitrary (g) . into_iter () . map (| d | d . 0) . skip_while (| d | d == & 0) . take (max_digits as usize) . collect :: < Vec < _ > > () ; while digits . last () == Some (& 0) { digits . pop () ; } digits }
};
}
