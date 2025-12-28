macro_rules! deps {
    () => {
        Create!();
    };
}

macro_rules! macro_107 {
    () => {
        deps!();
        trait_template ! { # [doc = " Create or access the adjacency matrix of a graph."] # [doc = ""] # [doc = " The implementor can either create an adjacency matrix, or it can return"] # [doc = " a placeholder if it has the needed representation internally."] # [allow (clippy :: needless_arbitrary_self_type)] pub trait GetAdjacencyMatrix : GraphBase { @ section type # [doc = " The associated adjacency matrix type"] type AdjMatrix ; @ section self # [doc = " Create the adjacency matrix"] fn adjacency_matrix (self : & Self) -> Self :: AdjMatrix ; # [doc = " Return true if there is an edge from `a` to `b`, false otherwise."] # [doc = ""] # [doc = " Computes in O(1) time."] fn is_adjacent (self : & Self , matrix : & Self :: AdjMatrix , a : Self :: NodeId , b : Self :: NodeId) -> bool ; } }
    };
}

macro_107!();