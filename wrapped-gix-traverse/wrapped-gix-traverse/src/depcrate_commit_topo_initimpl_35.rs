// Generated macro for impl_35 (impl)
macro_rules! Depcrate_commit_topo_initimpl_35 {
() => {
// Module: crate::commit::topo::init
// Provides: {"impl_35"}
// Dependencies: {}
impl < Find > Builder < Find , fn (& oid) -> bool > where Find : gix_object :: Find , { # [doc = " Create a new `Builder` for a [`Topo`] that reads commits from a repository with `find`."] # [doc = " starting at the `tips` and ending at the `ends`. Like `git rev-list"] # [doc = " --topo-order ^ends tips`."] pub fn from_iters (find : Find , tips : impl IntoIterator < Item = impl Into < ObjectId > > , ends : Option < impl IntoIterator < Item = impl Into < ObjectId > > > ,) -> Self { Self :: new (find) . with_tips (tips) . with_ends (ends . into_iter () . flatten ()) } # [doc = " Create a new `Builder` for a [`Topo`] that reads commits from a"] # [doc = " repository with `find`."] pub fn new (find : Find) -> Self { Self { commit_graph : Default :: default () , find , sorting : Default :: default () , parents : Default :: default () , tips : Default :: default () , ends : Default :: default () , predicate : | _ | true , } } # [doc = " Set a `predicate` to filter out revisions from the walk. Can be used to"] # [doc = " implement e.g. filtering on paths or time. This does *not* exclude the"] # [doc = " parent(s) of a revision that is excluded. Specify a revision as an 'end'"] # [doc = " if you want that behavior."] pub fn with_predicate < Predicate > (self , predicate : Predicate) -> Builder < Find , Predicate > where Predicate : FnMut (& oid) -> bool , { Builder { commit_graph : self . commit_graph , find : self . find , sorting : self . sorting , parents : self . parents , tips : self . tips , ends : self . ends , predicate , } } }
};
}
