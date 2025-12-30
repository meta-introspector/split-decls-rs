// Generated macro for Builder (struct)
macro_rules! Depcrate_commit_topo_initBuilder {
() => {
// Module: crate::commit::topo::init
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " Builder for [`Topo`]."] pub struct Builder < Find , Predicate > { commit_graph : Option < gix_commitgraph :: Graph > , find : Find , predicate : Predicate , sorting : Sorting , parents : Parents , tips : Vec < ObjectId > , ends : Vec < ObjectId > , }
};
}
