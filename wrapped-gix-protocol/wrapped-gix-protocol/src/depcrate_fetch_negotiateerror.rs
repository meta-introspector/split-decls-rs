// Generated macro for Error (enum)
macro_rules! Depcrate_fetch_negotiateError {
() => {
// Module: crate::fetch::negotiate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned during [`one_round()`] or [`mark_complete_and_common_ref()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("We were unable to figure out what objects the server should send after {rounds} round(s)")] NegotiationFailed { rounds : usize } , # [error (transparent)] LookupCommitInGraph (# [from] gix_revwalk :: graph :: get_or_insert_default :: Error) , # [error (transparent)] OpenPackedRefsBuffer (# [from] gix_ref :: packed :: buffer :: open :: Error) , # [error (transparent)] IO (# [from] std :: io :: Error) , # [error (transparent)] InitRefIter (# [from] gix_ref :: file :: iter :: loose_then_packed :: Error) , # [error (transparent)] PeelToId (# [from] gix_ref :: peel :: to_id :: Error) , # [error (transparent)] AlternateRefsAndObjects (Box < dyn std :: error :: Error + Send + Sync + 'static >) , }
};
}
