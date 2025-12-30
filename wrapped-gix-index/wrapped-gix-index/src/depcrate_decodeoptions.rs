// Generated macro for Options (struct)
macro_rules! Depcrate_decodeOptions {
() => {
// Module: crate::decode
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options to define how to decode an index state [from bytes][State::from_bytes()]."] # [derive (Debug , Default , Clone , Copy)] pub struct Options { # [doc = " If Some(_), we are allowed to use more than one thread. If Some(N), use no more than N threads. If Some(0)|None, use as many threads"] # [doc = " as there are logical cores."] # [doc = ""] # [doc = " This applies to loading extensions in parallel to entries if the common EOIE extension is available."] # [doc = " It also allows to use multiple threads for loading entries if the IEOT extension is present."] pub thread_limit : Option < usize > , # [doc = " The minimum size in bytes to load extensions in their own thread, assuming there is enough `num_threads` available."] # [doc = " If set to 0, for example, extensions will always be read in their own thread if enough threads are available."] pub min_extension_block_in_bytes_for_threading : usize , # [doc = " Set the expected hash of this index if we are read as part of a `link` extension."] # [doc = ""] # [doc = " We will abort reading this file if it doesn't match."] pub expected_checksum : Option < gix_hash :: ObjectId > , }
};
}
