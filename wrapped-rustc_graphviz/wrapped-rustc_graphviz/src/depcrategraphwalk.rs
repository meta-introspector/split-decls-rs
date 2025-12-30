// Generated macro for GraphWalk (trait)
macro_rules! DepcrateGraphWalk {
() => {
// Module: crate
// Provides: {"GraphWalk"}
// Dependencies: {}
# [doc = " GraphWalk is an abstraction over a directed graph = (nodes,edges)"] # [doc = " made up of node handles `N` and edge handles `E`, where each `E`"] # [doc = " can be mapped to its source and target nodes."] # [doc = ""] # [doc = " The lifetime parameter `'a` is exposed in this trait (rather than"] # [doc = " introduced as a generic parameter on each method declaration) so"] # [doc = " that a client impl can choose `N` and `E` that have substructure"] # [doc = " that is bound by the self lifetime `'a`."] # [doc = ""] # [doc = " The `nodes` and `edges` method each return instantiations of"] # [doc = " `Cow<[T]>` to leave implementors the freedom to create"] # [doc = " entirely new vectors or to pass back slices into internally owned"] # [doc = " vectors."] pub trait GraphWalk < 'a > { type Node : Clone ; type Edge : Clone ; # [doc = " Returns all the nodes in this graph."] fn nodes (& 'a self) -> Nodes < 'a , Self :: Node > ; # [doc = " Returns all of the edges in this graph."] fn edges (& 'a self) -> Edges < 'a , Self :: Edge > ; # [doc = " The source node for `edge`."] fn source (& 'a self , edge : & Self :: Edge) -> Self :: Node ; # [doc = " The target node for `edge`."] fn target (& 'a self , edge : & Self :: Edge) -> Self :: Node ; }
};
}
