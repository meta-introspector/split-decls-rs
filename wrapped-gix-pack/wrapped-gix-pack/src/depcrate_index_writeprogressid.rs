// Generated macro for ProgressId (enum)
macro_rules! Depcrate_index_writeProgressId {
() => {
// Module: crate::index::write
// Provides: {"ProgressId"}
// Dependencies: {}
# [doc = " The progress ids used in [`write_data_iter_from_stream()`][crate::index::File::write_data_iter_to_stream()]."] # [doc = ""] # [doc = " Use this information to selectively extract the progress of interest in case the parent application has custom visualization."] # [derive (Debug , Copy , Clone)] pub enum ProgressId { # [doc = " Counts the amount of objects that were index thus far."] IndexObjects , # [doc = " The amount of bytes that were decompressed while decoding pack entries."] # [doc = ""] # [doc = " This is done to determine entry boundaries."] DecompressedBytes , # [doc = " The amount of objects whose hashes were computed."] # [doc = ""] # [doc = " This is done by decoding them, which typically involves decoding delta objects."] ResolveObjects , # [doc = " The amount of bytes that were decoded in total, as the sum of all bytes to represent all resolved objects."] DecodedBytes , # [doc = " The amount of bytes written to the index file."] IndexBytesWritten , }
};
}
