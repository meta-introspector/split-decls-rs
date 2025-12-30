// Generated macro for Outcome (struct)
macro_rules! Depcrate_index_writeOutcome {
() => {
// Module: crate::index::write
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " Information gathered while executing [`write_data_iter_to_stream()`][crate::index::File::write_data_iter_to_stream]"] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Outcome { # [doc = " The version of the verified index"] pub index_version : crate :: index :: Version , # [doc = " The verified checksum of the verified index"] pub index_hash : gix_hash :: ObjectId , # [doc = " The hash of the '.pack' file, also found in its trailing bytes"] pub data_hash : gix_hash :: ObjectId , # [doc = " The amount of objects that were verified, always the amount of objects in the pack."] pub num_objects : u32 , }
};
}
