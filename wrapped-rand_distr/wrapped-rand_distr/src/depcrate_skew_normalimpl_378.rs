// Generated macro for impl_378 (impl)
macro_rules! Depcrate_skew_normalimpl_378 {
() => {
// Module: crate::skew_normal
// Provides: {"impl_378"}
// Dependencies: {}
impl < F > SkewNormal < F > where F : Float , StandardNormal : Distribution < F > , { # [doc = " Construct, from location, scale and shape."] # [doc = ""] # [doc = " Parameters:"] # [doc = ""] # [doc = " -   location (unrestricted)"] # [doc = " -   scale (must be finite and larger than zero)"] # [doc = " -   shape (must be finite)"] # [inline] pub fn new (location : F , scale : F , shape : F) -> Result < SkewNormal < F > , Error > { if ! scale . is_finite () || ! (scale > F :: zero ()) { return Err (Error :: ScaleTooSmall) ; } if ! shape . is_finite () { return Err (Error :: BadShape) ; } Ok (SkewNormal { location , scale , shape , }) } # [doc = " Returns the location of the distribution."] pub fn location (& self) -> F { self . location } # [doc = " Returns the scale of the distribution."] pub fn scale (& self) -> F { self . scale } # [doc = " Returns the shape of the distribution."] pub fn shape (& self) -> F { self . shape } }
};
}
