// Generated macro for Data (enum)
macro_rules! Depcrate_blob_pipelineData {
() => {
// Module: crate::blob::pipeline
// Provides: {"Data"}
// Dependencies: {}
# [doc = " Data as returned by [`Pipeline::convert_to_mergeable()`]."] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Debug)] pub enum Data { # [doc = " The data to use for merging was written into the buffer that was passed during the call to [`Pipeline::convert_to_mergeable()`]."] Buffer , # [doc = " The file or blob is above the big-file threshold and cannot be processed."] # [doc = ""] # [doc = " In this state, the file cannot be merged."] TooLarge { # [doc = " The size of the object prior to performing any filtering or as it was found on disk."] # [doc = ""] # [doc = " Note that technically, the size isn't always representative of the same 'state' of the"] # [doc = " content, as once it can be the size of the blob in git, and once it's the size of file"] # [doc = " in the worktree - both can differ a lot depending on filters."] size : u64 , } , }
};
}
