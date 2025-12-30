// Generated macro for Outcome (struct)
macro_rules! Depcrate_data_file_decode_entryOutcome {
() => {
// Module: crate::data::file::decode::entry
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " Additional information and statistics about a successfully decoded object produced by [`File::decode_entry()`]."] # [doc = ""] # [doc = " Useful to understand the effectiveness of the pack compression or the cost of decompression."] # [derive (Debug , PartialEq , Eq , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Outcome { # [doc = " The kind of resolved object."] pub kind : gix_object :: Kind , # [doc = " The amount of deltas in the chain of objects that had to be resolved beforehand."] # [doc = ""] # [doc = " This number is affected by the [`Cache`][cache::DecodeEntry] implementation, with cache hits shortening the"] # [doc = " delta chain accordingly"] pub num_deltas : u32 , # [doc = " The total decompressed size of all pack entries in the delta chain"] pub decompressed_size : u64 , # [doc = " The total compressed size of all pack entries in the delta chain"] pub compressed_size : usize , # [doc = " The total size of the decoded object."] pub object_size : u64 , }
};
}
