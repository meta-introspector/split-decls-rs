// Generated macro for impl_401 (impl)
macro_rules! Depcrate_triangularimpl_401 {
() => {
// Module: crate::triangular
// Provides: {"impl_401"}
// Dependencies: {}
impl < F > Distribution < F > for Triangular < F > where F : Float , StandardUniform : Distribution < F > , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { let f : F = rng . sample (StandardUniform) ; let diff_mode_min = self . mode - self . min ; let range = self . max - self . min ; let f_range = f * range ; if f_range < diff_mode_min { self . min + (f_range * diff_mode_min) . sqrt () } else { self . max - ((range - f_range) * (self . max - self . mode)) . sqrt () } } }
};
}
