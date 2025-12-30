// Generated macro for impl_231 (impl)
macro_rules! Depcrate_gammaimpl_231 {
() => {
// Module: crate::gamma
// Provides: {"impl_231"}
// Dependencies: {}
impl < F > Gamma < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { # [doc = " Construct an object representing the `Gamma(shape, scale)`"] # [doc = " distribution."] # [inline] pub fn new (shape : F , scale : F) -> Result < Gamma < F > , Error > { if ! (shape > F :: zero ()) { return Err (Error :: ShapeTooSmall) ; } if ! (scale > F :: zero ()) { return Err (Error :: ScaleTooSmall) ; } let repr = if shape == F :: one () { One (Exp :: new (F :: one () / scale) . map_err (| _ | Error :: ScaleTooLarge) ?) } else if shape < F :: one () { Small (GammaSmallShape :: new_raw (shape , scale)) } else { Large (GammaLargeShape :: new_raw (shape , scale)) } ; Ok (Gamma { repr }) } }
};
}
