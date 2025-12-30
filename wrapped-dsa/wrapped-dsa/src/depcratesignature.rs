// Generated macro for Signature (struct)
macro_rules! DepcrateSignature {
() => {
// Module: crate
// Provides: {"Signature"}
// Dependencies: {}
# [doc = " Container of the DSA signature"] # [derive (Clone , Debug)] # [must_use] pub struct Signature { # [doc = " Signature part r"] r : NonZero < BoxedUint > , # [doc = " Signature part s"] s : NonZero < BoxedUint > , }
};
}
