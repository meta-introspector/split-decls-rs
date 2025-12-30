// Generated macro for Spec (struct)
macro_rules! Depcrate_revisionSpec {
() => {
// Module: crate::revision
// Provides: {"Spec"}
// Dependencies: {}
# [doc = " The specification of a revision as parsed from a revision specification like `HEAD@{1}` or `v1.2.3...main`."] # [doc = " It's typically created by [`repo.rev_parse()`][crate::Repository::rev_parse()]."] # [doc = ""] # [doc = " See the [official git documentation](https://git-scm.com/docs/git-rev-parse#_specifying_revisions) for reference on how"] # [doc = " to specify revisions and revision ranges."] # [derive (Clone , Debug)] # [cfg (feature = "revision")] pub struct Spec < 'repo > { pub (crate) inner : gix_revision :: Spec , # [doc = " The path we encountered in the revspec, like `@:<path>` or `@..@~1:<path>`."] pub (crate) path : Option < (crate :: bstr :: BString , gix_object :: tree :: EntryMode) > , # [doc = " The first name of a reference as seen while parsing a `RevSpec`, for completeness."] pub (crate) first_ref : Option < gix_ref :: Reference > , # [doc = " The second name of a reference as seen while parsing a `RevSpec`, for completeness."] pub (crate) second_ref : Option < gix_ref :: Reference > , # [doc = " The owning repository."] pub repo : & 'repo crate :: Repository , }
};
}
