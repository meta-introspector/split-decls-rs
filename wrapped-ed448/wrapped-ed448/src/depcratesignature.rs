// Generated macro for Signature (struct)
macro_rules! DepcrateSignature {
() => {
// Module: crate
// Provides: {"Signature"}
// Dependencies: {}
# [doc = " Ed448 signature."] # [doc = ""] # [doc = " This type represents a container for the byte serialization of an Ed448"] # [doc = " signature, and does not necessarily represent well-formed field or curve"] # [doc = " elements."] # [doc = ""] # [doc = " Signature verification libraries are expected to reject invalid field"] # [doc = " elements at the time a signature is verified."] # [derive (Copy , Clone , Eq , PartialEq)] # [repr (C)] pub struct Signature { R : ComponentBytes , s : ComponentBytes , }
};
}
