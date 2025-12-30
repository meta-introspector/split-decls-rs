// Generated macro for error (module)
macro_rules! Depcrate_store_impls_dynamic_load_indexerror {
() => {
// Module: crate::store_impls::dynamic::load_index
// Provides: {"error"}
// Dependencies: {}
mod error { use std :: path :: PathBuf ; use gix_pack :: multi_index :: PackIndex ; # [doc = " Returned by [`crate::at_opts()`]"] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("The objects directory at '{0}' is not an accessible directory")] Inaccessible (PathBuf) , # [error (transparent)] Io (# [from] std :: io :: Error) , # [error (transparent)] Alternate (# [from] crate :: alternate :: Error) , # [error ("The slotmap turned out to be too small with {} entries, would need {} more" , . current , . needed)] InsufficientSlots { current : usize , needed : usize } , # [doc = " The problem here is that some logic assumes that more recent generations are higher than previous ones. If we would overflow,"] # [doc = " we would break that invariant which can lead to the wrong object from being returned. It would probably be super rare, but…"] # [doc = " let's not risk it."] # [error ("Would have overflown amount of max possible generations of {}" , super :: Generation :: MAX)] GenerationOverflow , # [error ("Cannot numerically handle more than {limit} packs in a single multi-pack index, got {actual} in file {index_path:?}")] TooManyPacksInMultiIndex { actual : PackIndex , limit : PackIndex , index_path : PathBuf , } , } }
};
}
