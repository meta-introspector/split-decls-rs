// Generated macro for SigningKey (struct)
macro_rules! DepcrateSigningKey {
() => {
// Module: crate
// Provides: {"SigningKey"}
// Dependencies: {}
# [doc = " An ML-DSA signing key"] # [derive (Clone , PartialEq)] pub struct SigningKey < P : MlDsaParams > { rho : B32 , K : B32 , tr : B64 , s1 : Vector < P :: L > , s2 : Vector < P :: K > , t0 : Vector < P :: K > , s1_hat : NttVector < P :: L > , s2_hat : NttVector < P :: K > , t0_hat : NttVector < P :: K > , A_hat : NttMatrix < P :: K , P :: L > , }
};
}
