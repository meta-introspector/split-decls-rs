// Generated macro for impl_232 (impl)
macro_rules! Depcrate_gammaimpl_232 {
() => {
// Module: crate::gamma
// Provides: {"impl_232"}
// Dependencies: {}
impl < F > GammaSmallShape < F > where F : Float , StandardNormal : Distribution < F > , Open01 : Distribution < F > , { fn new_raw (shape : F , scale : F) -> GammaSmallShape < F > { GammaSmallShape { inv_shape : F :: one () / shape , large_shape : GammaLargeShape :: new_raw (shape + F :: one () , scale) , } } }
};
}
