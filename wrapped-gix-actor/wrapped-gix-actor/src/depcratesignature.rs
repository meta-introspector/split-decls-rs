// Generated macro for Signature (struct)
macro_rules! DepcrateSignature {
() => {
// Module: crate
// Provides: {"Signature"}
// Dependencies: {}
# [doc = " A mutable signature that is created by an actor at a certain time."] # [doc = ""] # [doc = " Note that this is not a cryptographical signature."] # [derive (Default , PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Signature { # [doc = " The actors name, potentially with whitespace as parsed."] # [doc = ""] # [doc = " Use [SignatureRef::trim()] or trim manually to be able to clean it up."] pub name : BString , # [doc = " The actor's email, potentially with whitespace and garbage as parsed."] # [doc = ""] # [doc = " Use [SignatureRef::trim()] or trim manually to be able to clean it up."] pub email : BString , # [doc = " The time stamp at which the signature is performed."] pub time : date :: Time , }
};
}
