// Generated macro for emit (module)
macro_rules! Depcrate_rewrites_trackeremit {
() => {
// Module: crate::rewrites::tracker
// Provides: {"emit"}
// Dependencies: {}
# [doc = ""] pub mod emit { # [doc = " The error returned by [Tracker::emit()](super::Tracker::emit())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not find blob for similarity checking")] FindExistingBlob (# [from] gix_object :: find :: existing_object :: Error) , # [error ("Could not obtain exhaustive item set to use as possible sources for copy detection")] GetItemsForExhaustiveCopyDetection (# [source] Box < dyn std :: error :: Error + Send + Sync >) , # [error (transparent)] SetResource (# [from] crate :: blob :: platform :: set_resource :: Error) , # [error (transparent)] PrepareDiff (# [from] crate :: blob :: platform :: prepare_diff :: Error) , } }
};
}
