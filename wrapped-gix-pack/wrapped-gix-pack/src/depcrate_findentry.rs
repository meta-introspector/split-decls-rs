// Generated macro for Entry (struct)
macro_rules! Depcrate_findEntry {
() => {
// Module: crate::find
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " An Entry in a pack providing access to its data."] # [doc = ""] # [doc = " Its commonly retrieved by reading from a pack index file followed by a read from a pack data file."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (missing_docs)] pub struct Entry { # [doc = " The pack-data encoded bytes of the pack data entry as present in the pack file, including the header followed by compressed data."] pub data : Vec < u8 > , # [doc = " The version of the pack file containing `data`"] pub version : crate :: data :: Version , }
};
}
