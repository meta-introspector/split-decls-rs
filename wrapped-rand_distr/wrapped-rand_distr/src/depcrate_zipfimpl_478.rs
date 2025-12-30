// Generated macro for impl_478 (impl)
macro_rules! Depcrate_zipfimpl_478 {
() => {
// Module: crate::zipf
// Provides: {"impl_478"}
// Dependencies: {}
impl < F > Zipf < F > where F : Float , StandardUniform : Distribution < F > , { # [doc = " Construct a new `Zipf` distribution for a set with `n` elements and a"] # [doc = " frequency rank exponent `s`."] # [doc = ""] # [doc = " The parameter `n` is typically integral, however we use type"] # [doc = " <pre><code>F: [Float]</code></pre> in order to permit very large values"] # [doc = " and since our implementation requires a floating-point type."] # [inline] pub fn new (n : F , s : F) -> Result < Zipf < F > , Error > { if ! (s >= F :: zero ()) { return Err (Error :: STooSmall) ; } if n < F :: one () { return Err (Error :: NTooSmall) ; } let q = if s != F :: one () { F :: one () / (F :: one () - s) } else { F :: zero () } ; let t = if s != F :: one () { (n . powf (F :: one () - s) - s) * q } else { F :: one () + n . ln () } ; debug_assert ! (t > F :: zero ()) ; Ok (Zipf { s , t , q }) } # [doc = " Inverse cumulative density function"] # [inline] fn inv_cdf (& self , p : F) -> F { let one = F :: one () ; let pt = p * self . t ; if pt <= one { pt } else if self . s != one { (pt * (one - self . s) + self . s) . powf (self . q) } else { (pt - one) . exp () } } }
};
}
