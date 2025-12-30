// Generated macro for ProgressId (enum)
macro_rules! Depcrate_index_traverse_with_indexProgressId {
() => {
// Module: crate::index::traverse::with_index
// Provides: {"ProgressId"}
// Dependencies: {}
# [doc = " The progress ids used in [`index::File::traverse_with_index()`]."] # [doc = ""] # [doc = " Use this information to selectively extract the progress of interest in case the parent application has custom visualization."] # [derive (Debug , Copy , Clone)] pub enum ProgressId { # [doc = " The amount of bytes currently processed to generate a checksum of the *pack data file*."] HashPackDataBytes , # [doc = " The amount of bytes currently processed to generate a checksum of the *pack index file*."] HashPackIndexBytes , # [doc = " Collect all object hashes into a vector and sort it by their pack offset."] CollectSortedIndexEntries , # [doc = " Count the objects processed when building a cache tree from all objects in a pack index."] TreeFromOffsetsObjects , # [doc = " The amount of objects which were decoded."] DecodedObjects , # [doc = " The amount of bytes that were decoded in total, as the sum of all bytes to represent all decoded objects."] DecodedBytes , }
};
}
