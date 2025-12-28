macro_rules! deps {
    () => {
        Matching!();
        MatchedEdges!();
        MatchedNodes!();
    };
}

macro_rules! impl_399 {
    () => {
        deps!();
        impl < G > Matching < G > where G : NodeIndexable , { # [doc = " Gets the matched counterpart of given node, if there is any."] # [doc = ""] # [doc = " Returns `None` if the node is not matched or does not exist."] pub fn mate (& self , node : G :: NodeId) -> Option < G :: NodeId > { self . mate . get (self . graph . to_index (node)) . and_then (| & id | id) } # [doc = " Iterates over all edges from the matching."] # [doc = ""] # [doc = " An edge is represented by its endpoints. The graph is considered"] # [doc = " undirected and every pair of matched nodes is reported only once."] pub fn edges (& self) -> MatchedEdges < '_ , G > { MatchedEdges { graph : & self . graph , mate : self . mate . as_slice () , current : 0 , } } # [doc = " Iterates over all nodes from the matching."] pub fn nodes (& self) -> MatchedNodes < '_ , G > { MatchedNodes { graph : & self . graph , mate : self . mate . as_slice () , current : 0 , } } # [doc = " Returns `true` if given edge is in the matching, or `false` otherwise."] # [doc = ""] # [doc = " If any of the nodes does not exist, `false` is returned."] pub fn contains_edge (& self , a : G :: NodeId , b : G :: NodeId) -> bool { match self . mate (a) { Some (mate) => mate == b , None => false , } } # [doc = " Returns `true` if given node is in the matching, or `false` otherwise."] # [doc = ""] # [doc = " If the node does not exist, `false` is returned."] pub fn contains_node (& self , node : G :: NodeId) -> bool { self . mate (node) . is_some () } # [doc = " Gets the number of matched **edges**."] pub fn len (& self) -> usize { self . n_edges } # [doc = " Returns `true` if the number of matched **edges** is 0."] pub fn is_empty (& self) -> bool { self . len () == 0 } }
    };
}

impl_399!();