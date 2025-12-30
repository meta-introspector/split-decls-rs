// Generated macro for Platform (struct)
macro_rules! Depcrate_revision_walkPlatform {
() => {
// Module: crate::revision::walk
// Provides: {"Platform"}
// Dependencies: {}
# [doc = " A platform to traverse the revision graph by adding starting points as well as points which shouldn't be crossed,"] # [doc = " returned by [`Repository::rev_walk()`]."] # [doc = ""] # [doc = " **Note that we automatically leverage the commitgraph data structure**, but if you know that additional information like"] # [doc = " author or commit messages will be required of *all* commits traversed here, it should be better to avoid trying to load it"] # [doc = " by [turning commit-graph support off][Platform::use_commit_graph()]. This certainly is a micro-optimization though."] pub struct Platform < 'repo > { # [doc = " The owning repository."] pub repo : & 'repo Repository , pub (crate) tips : Vec < ObjectId > , pub (crate) hidden : Vec < ObjectId > , pub (crate) boundary : Vec < ObjectId > , pub (crate) sorting : Sorting , pub (crate) parents : gix_traverse :: commit :: Parents , pub (crate) use_commit_graph : Option < bool > , pub (crate) commit_graph : Option < gix_commitgraph :: Graph > , }
};
}
