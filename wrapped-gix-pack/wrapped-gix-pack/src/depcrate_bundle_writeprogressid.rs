// Generated macro for ProgressId (enum)
macro_rules! Depcrate_bundle_writeProgressId {
() => {
// Module: crate::bundle::write
// Provides: {"ProgressId"}
// Dependencies: {}
# [doc = " The progress ids used in [`write_to_directory()`][crate::Bundle::write_to_directory()]."] # [doc = ""] # [doc = " Use this information to selectively extract the progress of interest in case the parent application has custom visualization."] # [derive (Debug , Copy , Clone)] pub enum ProgressId { # [doc = " The amount of bytes read from the input pack data file."] ReadPackBytes , # [doc = " A root progress counting logical steps towards an index file on disk."] # [doc = ""] # [doc = " Underneath will be more progress information related to actually producing the index."] IndexingSteps (PhantomData < crate :: index :: write :: ProgressId >) , }
};
}
