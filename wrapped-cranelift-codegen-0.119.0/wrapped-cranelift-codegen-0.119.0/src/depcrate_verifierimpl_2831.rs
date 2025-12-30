// Generated macro for impl_2831 (impl)
macro_rules! Depcrate_verifierimpl_2831 {
() => {
// Module: crate::verifier
// Provides: {"impl_2831"}
// Dependencies: {}
# [doc = " Convenience converter for making error-reporting less verbose."] # [doc = ""] # [doc = " Same as above but without `context`."] impl < L , M > From < (L , M) > for VerifierError where L : Into < AnyEntity > , M : Into < String > , { fn from (items : (L , M)) -> Self { let (location , message) = items ; Self { location : location . into () , context : None , message : message . into () , } } }
};
}
