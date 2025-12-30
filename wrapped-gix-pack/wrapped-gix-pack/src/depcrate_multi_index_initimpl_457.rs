// Generated macro for impl_457 (impl)
macro_rules! Depcrate_multi_index_initimpl_457 {
() => {
// Module: crate::multi_index::init
// Provides: {"impl_457"}
// Dependencies: {}
# [doc = " Initialization"] impl File { # [doc = " Open the multi-index file at the given `path`."] pub fn at (path : impl AsRef < Path >) -> Result < Self , Error > { Self :: try_from (path . as_ref ()) } }
};
}
