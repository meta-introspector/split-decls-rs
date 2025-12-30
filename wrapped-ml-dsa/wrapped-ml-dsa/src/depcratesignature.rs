// Generated macro for Signature (struct)
macro_rules! DepcrateSignature {
() => {
// Module: crate
// Provides: {"Signature"}
// Dependencies: {}
# [doc = " An ML-DSA signature"] # [derive (Clone , PartialEq , Debug)] pub struct Signature < P : MlDsaParams > { c_tilde : Array < u8 , P :: Lambda > , z : Vector < P :: L > , h : Hint < P > , }
};
}
