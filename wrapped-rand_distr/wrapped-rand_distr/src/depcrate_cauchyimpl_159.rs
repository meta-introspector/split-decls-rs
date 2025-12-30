// Generated macro for impl_159 (impl)
macro_rules! Depcrate_cauchyimpl_159 {
() => {
// Module: crate::cauchy
// Provides: {"impl_159"}
// Dependencies: {}
impl < F > Distribution < F > for Cauchy < F > where F : Float + FloatConst , StandardUniform : Distribution < F > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { let x = StandardUniform . sample (rng) ; let comp_dev = (F :: PI () * x) . tan () ; self . median + self . scale * comp_dev } }
};
}
