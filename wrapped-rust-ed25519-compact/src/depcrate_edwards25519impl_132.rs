// Generated macro for impl_132 (impl)
macro_rules! Depcrate_edwards25519impl_132 {
() => {
// Module: crate::edwards25519
// Provides: {"impl_132"}
// Dependencies: {}
impl GeP3 { pub fn from_bytes_negate_vartime (s : & [u8 ; 32]) -> Option < GeP3 > { let y = Fe :: from_bytes (s) ; let z = FE_ONE ; let y_squared = y . square () ; let u = y_squared - FE_ONE ; let v = (y_squared * FE_D) + FE_ONE ; let mut x = (u * v) . pow25523 () * u ; let vxx = x . square () * v ; let check = vxx - u ; if ! check . is_zero () { let check2 = vxx + u ; if ! check2 . is_zero () { return None ; } x = x * FE_SQRTM1 ; } if x . is_negative () == ((s [31] >> 7) != 0) { x = x . neg () ; } let t = x * y ; Some (GeP3 { x , y , z , t }) } pub fn from_bytes_vartime (s : & [u8 ; 32]) -> Option < GeP3 > { Self :: from_bytes_negate_vartime (s) . map (| p | GeP3 { x : p . x . neg () , y : p . y , z : p . z , t : p . t . neg () , }) } fn to_p2 (& self) -> GeP2 { GeP2 { x : self . x , y : self . y , z : self . z , } } fn to_cached (& self) -> GeCached { GeCached { y_plus_x : self . y + self . x , y_minus_x : self . y - self . x , z : self . z , t2d : self . t * FE_D2 , } } fn zero () -> GeP3 { GeP3 { x : FE_ZERO , y : FE_ONE , z : FE_ONE , t : FE_ZERO , } } fn dbl (& self) -> GeP1P1 { self . to_p2 () . dbl () } pub fn to_bytes (& self) -> [u8 ; 32] { let recip = self . z . invert () ; let x = self . x * recip ; let y = self . y * recip ; let mut bs = y . to_bytes () ; bs [31] ^= (if x . is_negative () { 1 } else { 0 }) << 7 ; bs } pub fn has_small_order (& self) -> bool { let recip = self . z . invert () ; let x = self . x * recip ; let y = self . y * recip ; let x_neg = x . neg () ; let y_sqrtm1 = y * FE_SQRTM1 ; x . is_zero () | y . is_zero () | (y_sqrtm1 == x) | (y_sqrtm1 == x_neg) } }
};
}
