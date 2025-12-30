// Generated macro for CandidateInfo (enum)
macro_rules! Depcrate_revision_spec_parse_errorCandidateInfo {
() => {
// Module: crate::revision::spec::parse::error
// Provides: {"CandidateInfo"}
// Dependencies: {}
# [doc = " Additional information about candidates that caused ambiguity."] # [derive (Debug)] pub enum CandidateInfo { # [doc = " An error occurred when looking up the object in the database."] FindError { # [doc = " The reported error."] source : crate :: object :: find :: existing :: Error , } , # [doc = " The candidate is an object of the given `kind`."] Object { # [doc = " The kind of the object."] kind : gix_object :: Kind , } , # [doc = " The candidate is a tag."] Tag { # [doc = " The name of the tag."] name : BString , } , # [doc = " The candidate is a commit."] Commit { # [doc = " The date of the commit."] date : String , # [doc = " The subject line."] title : BString , } , }
};
}
