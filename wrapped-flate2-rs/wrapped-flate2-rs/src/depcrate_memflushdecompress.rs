// Generated macro for FlushDecompress (enum)
macro_rules! Depcrate_memFlushDecompress {
() => {
// Module: crate::mem
// Provides: {"FlushDecompress"}
// Dependencies: {}
# [doc = " Values which indicate the form of flushing to be used when"] # [doc = " decompressing in-memory data."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] # [non_exhaustive] # [allow (clippy :: unnecessary_cast)] pub enum FlushDecompress { # [doc = " A typical parameter for passing to compression/decompression functions,"] # [doc = " this indicates that the underlying stream to decide how much data to"] # [doc = " accumulate before producing output in order to maximize compression."] None = ffi :: MZ_NO_FLUSH as isize , # [doc = " All pending output is flushed to the output buffer and the output is"] # [doc = " aligned on a byte boundary so that the decompressor can get all input"] # [doc = " data available so far."] # [doc = ""] # [doc = " Flushing may degrade compression for some compression algorithms and so"] # [doc = " it should only be used when necessary. This will complete the current"] # [doc = " deflate block and follow it with an empty stored block."] Sync = ffi :: MZ_SYNC_FLUSH as isize , # [doc = " Pending input is processed and pending output is flushed."] # [doc = ""] # [doc = " The return value may indicate that the stream is not yet done and more"] # [doc = " data has yet to be processed."] Finish = ffi :: MZ_FINISH as isize , }
};
}
