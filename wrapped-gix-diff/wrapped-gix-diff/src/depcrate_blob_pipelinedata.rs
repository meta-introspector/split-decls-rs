// Generated macro for Data (enum)
macro_rules! Depcrate_blob_pipelineData {
() => {
// Module: crate::blob::pipeline
// Provides: {"Data"}
// Dependencies: {}
# [doc = " Data as part of an [Outcome]."] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Debug)] pub enum Data { # [doc = " The data to use for diffing was written into the buffer that was passed during the call to [`Pipeline::convert_to_diffable()`]."] Buffer { # [doc = " If `true`, a [binary to text filter](Driver::binary_to_text_command) was used to obtain the buffer,"] # [doc = " making it a derived value."] # [doc = ""] # [doc = " Applications should check for this to avoid treating the buffer content as (original) resource content."] is_derived : bool , } , # [doc = " The size that the binary blob had at the given revision, without having applied filters, as it's either"] # [doc = " considered binary or above the big-file threshold."] # [doc = ""] # [doc = " In this state, the binary file cannot be diffed."] Binary { # [doc = " The size of the object prior to performing any filtering or as it was found on disk."] # [doc = ""] # [doc = " Note that technically, the size isn't always representative of the same 'state' of the"] # [doc = " content, as once it can be the size of the blob in git, and once it's the size of file"] # [doc = " in the worktree."] size : u64 , } , }
};
}
