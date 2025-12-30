// Generated macro for VerifyingKey (struct)
macro_rules! Depcrate_verifying_keyVerifyingKey {
() => {
// Module: crate::verifying_key
// Provides: {"VerifyingKey"}
// Dependencies: {}
# [doc = " DSA public key."] # [derive (Clone , Debug , PartialEq , PartialOrd)] # [must_use] pub struct VerifyingKey { # [doc = " common components"] components : Components , # [doc = " Public component y"] y : NonZero < BoxedUint > , }
};
}
