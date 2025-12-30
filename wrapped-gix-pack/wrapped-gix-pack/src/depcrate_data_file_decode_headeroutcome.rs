// Generated macro for Outcome (struct)
macro_rules! Depcrate_data_file_decode_headerOutcome {
() => {
// Module: crate::data::file::decode::header
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " Additional information and statistics about a successfully decoded object produced by [`File::decode_header()`]."] # [doc = ""] # [doc = " Useful to understand the effectiveness of the pack compression or the cost of decompression."] # [derive (Debug , PartialEq , Eq , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Outcome { # [doc = " The kind of resolved object."] pub kind : gix_object :: Kind , # [doc = " The decompressed size of the object."] pub object_size : u64 , # [doc = " The amount of deltas in the chain of objects that had to be resolved beforehand."] pub num_deltas : u32 , }
};
}
