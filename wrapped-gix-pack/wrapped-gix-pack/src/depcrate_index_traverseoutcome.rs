// Generated macro for Outcome (struct)
macro_rules! Depcrate_index_traverseOutcome {
() => {
// Module: crate::index::traverse
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " The outcome of the [`traverse()`][index::File::traverse()] method."] pub struct Outcome { # [doc = " The checksum obtained when hashing the file, which matched the checksum contained within the file."] pub actual_index_checksum : gix_hash :: ObjectId , # [doc = " The statistics obtained during traversal."] pub statistics : Statistics , }
};
}
