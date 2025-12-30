// Generated macro for Info (struct)
macro_rules! Depcrate_revision_walkInfo {
() => {
// Module: crate::revision::walk
// Provides: {"Info"}
// Dependencies: {}
# [doc = " Information about a commit that we obtained naturally as part of the iteration."] # [derive (Debug , Clone)] pub struct Info < 'repo > { # [doc = " The detached id of the commit."] pub id : gix_hash :: ObjectId , # [doc = " All parent ids we have encountered. Note that these will be at most one if [`Parents::First`][gix_traverse::commit::Parents::First] is enabled."] pub parent_ids : gix_traverse :: commit :: ParentIds , # [doc = " The time at which the commit was created. It will only be `Some(_)` if the chosen traversal was"] # [doc = " taking dates into consideration."] pub commit_time : Option < gix_date :: SecondsSinceUnixEpoch > , repo : & 'repo Repository , }
};
}
