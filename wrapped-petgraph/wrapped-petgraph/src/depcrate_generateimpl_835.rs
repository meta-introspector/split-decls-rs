// Generated macro for impl_835 (impl)
macro_rules! Depcrate_generateimpl_835 {
() => {
// Module: crate::generate
// Provides: {"impl_835"}
// Dependencies: {}
impl Generator < Directed > { # [doc = " Generate all possible Directed acyclic graphs (DAGs) of a particular number of nodes."] # [doc = ""] # [doc = " These are only generated with one per isomorphism, so they use"] # [doc = " one canonical node labeling where node *i* can only have edges to node *j* if *i < j*."] # [doc = ""] # [doc = " For a graph of *k* nodes there are *e = (k - 1) k / 2* possible edges and"] # [doc = " *2<sup>e</sup>* DAGs."] pub fn directed_acyclic (nodes : usize) -> Self { assert ! (nodes != 0) ; let nedges = (nodes - 1) * nodes / 2 ; assert ! (nedges < 64) ; Generator { acyclic : true , selfloops : false , nodes , nedges , bits : ! 0 , g : Graph :: with_capacity (nodes , nedges) , } } }
};
}
