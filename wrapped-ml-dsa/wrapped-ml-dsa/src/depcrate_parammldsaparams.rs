// Generated macro for MlDsaParams (trait)
macro_rules! Depcrate_paramMlDsaParams {
() => {
// Module: crate::param
// Provides: {"MlDsaParams"}
// Dependencies: {}
# [doc = " An instance of `MlDsaParams` defines all of the parameters necessary for ML-DSA operations."] # [doc = " Typically this is done by implementing `ParameterSet` with values that will fit into the"] # [doc = " blanket implementations of `SigningKeyParams`, `VerifyingKeyParams`, and `SignatureParams`."] pub trait MlDsaParams : SigningKeyParams + VerifyingKeyParams + SignatureParams + Debug + Default + PartialEq + Clone { }
};
}
