// Generated macro for util (module)
macro_rules! Depcrate_data_output_entry_iter_from_countsutil {
() => {
// Module: crate::data::output::entry::iter_from_counts
// Provides: {"util"}
// Dependencies: {}
mod util { # [derive (Clone)] pub struct ChunkRanges { cursor : usize , size : usize , len : usize , } impl ChunkRanges { pub fn new (size : usize , total : usize) -> Self { ChunkRanges { cursor : 0 , size , len : total , } } } impl Iterator for ChunkRanges { type Item = std :: ops :: Range < usize > ; fn next (& mut self) -> Option < Self :: Item > { if self . cursor >= self . len { None } else { let upper = (self . cursor + self . size) . min (self . len) ; let range = self . cursor .. upper ; self . cursor = upper ; Some (range) } } } }
};
}
