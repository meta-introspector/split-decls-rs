// Generated macro for Token (enum)
macro_rules! Depcrate_commit_ref_iterToken {
() => {
// Module: crate::commit::ref_iter
// Provides: {"Token"}
// Dependencies: {}
# [doc = " A token returned by the [commit iterator][CommitRefIter]."] # [allow (missing_docs)] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] pub enum Token < 'a > { Tree { id : ObjectId , } , Parent { id : ObjectId , } , # [doc = " A person who authored the content of the commit."] Author { signature : gix_actor :: SignatureRef < 'a > , } , # [doc = " A person who committed the authors work to the repository."] Committer { signature : gix_actor :: SignatureRef < 'a > , } , Encoding (& 'a BStr) , ExtraHeader ((& 'a BStr , Cow < 'a , BStr >)) , Message (& 'a BStr) , }
};
}
