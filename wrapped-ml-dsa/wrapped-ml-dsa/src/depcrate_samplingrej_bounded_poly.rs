// Generated macro for rej_bounded_poly (function)
macro_rules! Depcrate_samplingrej_bounded_poly {
() => {
// Module: crate::sampling
// Provides: {"rej_bounded_poly"}
// Dependencies: {}
fn rej_bounded_poly (rho : & [u8] , eta : Eta , r : u16) -> Polynomial { let mut j = 0 ; let mut ctx = H :: default () . absorb (rho) . absorb (& r . to_le_bytes ()) ; let mut a = Polynomial :: default () ; let mut z = [0u8] ; while j < 256 { ctx . squeeze (& mut z) ; let (z0 , z1) = coeffs_from_byte (z [0] , eta) ; if let Some (z) = z0 { a . 0 [j] = z ; j += 1 ; } if j == 256 { break ; } if let Some (z) = z1 { a . 0 [j] = z ; j += 1 ; } } a }
};
}
