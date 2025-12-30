// Generated macro for impl_158 (impl)
macro_rules! Depcrate_cauchyimpl_158 {
() => {
// Module: crate::cauchy
// Provides: {"impl_158"}
// Dependencies: {}
impl < F > Cauchy < F > where F : Float + FloatConst , StandardUniform : Distribution < F > , { # [doc = " Construct a new `Cauchy` with the given shape parameters"] # [doc = " `median` the peak location and `scale` the scale factor."] pub fn new (median : F , scale : F) -> Result < Cauchy < F > , Error > { if ! (scale > F :: zero ()) { return Err (Error :: ScaleTooSmall) ; } Ok (Cauchy { median , scale }) } }
};
}
