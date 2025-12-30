// Generated macro for impl_400 (impl)
macro_rules! Depcrate_triangularimpl_400 {
() => {
// Module: crate::triangular
// Provides: {"impl_400"}
// Dependencies: {}
impl < F > Triangular < F > where F : Float , StandardUniform : Distribution < F > , { # [doc = " Set up the Triangular distribution with defined `min`, `max` and `mode`."] # [inline] pub fn new (min : F , max : F , mode : F) -> Result < Triangular < F > , TriangularError > { if ! (max >= min) { return Err (TriangularError :: RangeTooSmall) ; } if ! (mode >= min && max >= mode) { return Err (TriangularError :: ModeRange) ; } Ok (Triangular { min , max , mode }) } }
};
}
