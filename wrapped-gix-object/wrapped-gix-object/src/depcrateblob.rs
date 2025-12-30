// Generated macro for Blob (struct)
macro_rules! DepcrateBlob {
() => {
// Module: crate
// Provides: {"Blob"}
// Dependencies: {}
# [doc = " A mutable chunk of any [`data`](Blob::data)."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Blob { # [doc = " The data itself."] pub data : Vec < u8 > , }
};
}
