// Generated macro for impl_458 (impl)
macro_rules! Depcrate_zetaimpl_458 {
() => {
// Module: crate::zeta
// Provides: {"impl_458"}
// Dependencies: {}
impl < F > Zeta < F > where F : Float , StandardUniform : Distribution < F > , OpenClosed01 : Distribution < F > , { # [doc = " Construct a new `Zeta` distribution with given `s` parameter."] # [inline] pub fn new (s : F) -> Result < Zeta < F > , Error > { if ! (s > F :: one ()) { return Err (Error :: STooSmall) ; } let s_minus_1 = s - F :: one () ; let two = F :: one () + F :: one () ; Ok (Zeta { s_minus_1 , b : two . powf (s_minus_1) , }) } }
};
}
