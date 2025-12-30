// Generated macro for Error (enum)
macro_rules! Depcrate_cache_deltaError {
() => {
// Module: crate::cache::delta
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Returned when using various methods on a [`Tree`]"] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("Pack offsets must only increment. The previous pack offset was {last_pack_offset}, the current one is {pack_offset}")] InvariantIncreasingPackOffset { # [doc = " The last seen pack offset"] last_pack_offset : crate :: data :: Offset , # [doc = " The invariant violating offset"] pack_offset : crate :: data :: Offset , } , }
};
}
