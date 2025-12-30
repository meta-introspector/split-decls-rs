// Generated macro for impl_333 (impl)
macro_rules! Depcrate_paretoimpl_333 {
() => {
// Module: crate::pareto
// Provides: {"impl_333"}
// Dependencies: {}
impl < F > Pareto < F > where F : Float , OpenClosed01 : Distribution < F > , { # [doc = " Construct a new Pareto distribution with given `scale` and `shape`."] # [doc = ""] # [doc = " In the literature, `scale` is commonly written as x<sub>m</sub> or k and"] # [doc = " `shape` is often written as α."] pub fn new (scale : F , shape : F) -> Result < Pareto < F > , Error > { let zero = F :: zero () ; if ! (scale > zero) { return Err (Error :: ScaleTooSmall) ; } if ! (shape > zero) { return Err (Error :: ShapeTooSmall) ; } Ok (Pareto { scale , inv_neg_shape : F :: from (- 1.0) . unwrap () / shape , }) } }
};
}
