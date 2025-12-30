// Generated macro for impl_479 (impl)
macro_rules! Depcrate_zipfimpl_479 {
() => {
// Module: crate::zipf
// Provides: {"impl_479"}
// Dependencies: {}
impl < F > Distribution < F > for Zipf < F > where F : Float , StandardUniform : Distribution < F > , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { let one = F :: one () ; loop { let inv_b = self . inv_cdf (rng . sample (StandardUniform)) ; let x = (inv_b + one) . floor () ; let mut ratio = x . powf (- self . s) ; if x > one { ratio = ratio * inv_b . powf (self . s) } ; let y = rng . sample (StandardUniform) ; if y < ratio { return x ; } } } }
};
}
