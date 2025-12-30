// Generated macro for impl_233 (impl)
macro_rules! Depcrate_gammaimpl_233 {
() => {
// Module: crate::gamma
// Provides: {"impl_233"}
// Dependencies: {}
impl < F > GammaLargeShape < F > where F : Float , StandardNormal : Distribution < F > , Open01 : Distribution < F > , { fn new_raw (shape : F , scale : F) -> GammaLargeShape < F > { let d = shape - F :: from (1. / 3.) . unwrap () ; GammaLargeShape { scale , c : F :: one () / (F :: from (9.) . unwrap () * d) . sqrt () , d , } } }
};
}
