// Generated macro for SignatureRef (struct)
macro_rules! DepcrateSignatureRef {
() => {
// Module: crate
// Provides: {"SignatureRef"}
// Dependencies: {}
# [doc = " An immutable signature that is created by an actor at a certain time."] # [doc = ""] # [doc = " All of its fields are references to the backing buffer to allow lossless"] # [doc = " round-tripping, as decoding the `time` field could be a lossy transformation."] # [doc = ""] # [doc = " Note that this is not a cryptographical signature."] # [derive (Default , PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct SignatureRef < 'a > { # [doc = " The actors name, potentially with whitespace as parsed."] # [doc = ""] # [doc = " Use [SignatureRef::trim()] or trim manually for cleanup."] # [cfg_attr (feature = "serde" , serde (borrow))] pub name : & 'a BStr , # [doc = " The actor's email, potentially with whitespace and garbage as parsed."] # [doc = ""] # [doc = " Use [SignatureRef::trim()] or trim manually for cleanup."] pub email : & 'a BStr , # [doc = " The timestamp at which the signature was performed,"] # [doc = " potentially malformed due to lenient parsing."] # [doc = ""] # [doc = " Use [`SignatureRef::time()`] to decode."] pub time : & 'a str , }
};
}
