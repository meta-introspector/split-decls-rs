// Generated macro for VerifyingKey (struct)
macro_rules! DepcrateVerifyingKey {
() => {
// Module: crate
// Provides: {"VerifyingKey"}
// Dependencies: {}
# [doc = " An ML-DSA verification key"] # [derive (Clone , Debug , PartialEq)] pub struct VerifyingKey < P : ParameterSet > { rho : B32 , t1 : Vector < P :: K > , A_hat : NttMatrix < P :: K , P :: L > , t1_2d_hat : NttVector < P :: K > , tr : B64 , }
};
}
