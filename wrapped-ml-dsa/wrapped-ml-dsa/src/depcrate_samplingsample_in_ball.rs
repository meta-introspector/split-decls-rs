// Generated macro for sample_in_ball (function)
macro_rules! Depcrate_samplingsample_in_ball {
() => {
// Module: crate::sampling
// Provides: {"sample_in_ball"}
// Dependencies: {}
pub (crate) fn sample_in_ball (rho : & [u8] , tau : usize) -> Polynomial { const ONE : Elem = Elem :: new (1) ; const MINUS_ONE : Elem = Elem :: new (BaseField :: Q - 1) ; let mut c = Polynomial :: default () ; let mut ctx = H :: default () . absorb (rho) ; let mut s = [0u8 ; 8] ; ctx . squeeze (& mut s) ; let mut j = [0u8] ; for i in (256 - tau) .. 256 { ctx . squeeze (& mut j) ; while usize :: from (j [0]) > i { ctx . squeeze (& mut j) ; } let j = usize :: from (j [0]) ; c . 0 [i] = c . 0 [j] ; c . 0 [j] = if bit_set (& s , i + tau - 256) { MINUS_ONE } else { ONE } ; } c }
};
}
