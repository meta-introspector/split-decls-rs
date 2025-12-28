macro_rules! deps {
    () => {
        CompactDirection!();
        DiGraphMap!();
        Edge!();
        UnGraphMap!();
    };
}

macro_rules! GraphMap {
    () => {
        deps!();
        # [doc = " `GraphMap<N, E, Ty>` is a graph datastructure using an associative array"] # [doc = " of its node weights `N`."] # [doc = ""] # [doc = " It uses an combined adjacency list and sparse adjacency matrix"] # [doc = " representation, using **O(|V| + |E|)** space where V is the set of nodes"] # [doc = " and E is the set of edges, and allows testing for edge"] # [doc = " existence in constant time."] # [doc = ""] # [doc = " `GraphMap` is parameterized over:"] # [doc = ""] # [doc = " - Associated data `N` for nodes and `E` for edges, called *weights*."] # [doc = " - The node weight `N` must implement `Copy` and will be used as node"] # [doc = "   identifier, duplicated into several places in the data structure."] # [doc = "   It must be suitable as a hash table key (implementing `Eq + Hash`)."] # [doc = "   The node type must also implement `Ord` so that the implementation can"] # [doc = "   order the pair (`a`, `b`) for an edge connecting any two nodes `a` and `b`."] # [doc = " - `E` can be of arbitrary type."] # [doc = " - Edge type `Ty` that determines whether the graph edges are directed or"] # [doc = "   undirected."] # [doc = ""] # [doc = " You can use the type aliases `UnGraphMap` and `DiGraphMap` for convenience."] # [doc = ""] # [doc = " `GraphMap` does not allow parallel edges, but self loops are allowed."] # [doc = ""] # [doc = " Depends on crate feature `graphmap` (default)."] # [derive (Clone)] pub struct GraphMap < N , E , Ty , # [cfg (not (feature = "std"))] S , # [cfg (feature = "std")] S = RandomState , > where S : BuildHasher , { nodes : IndexMap < N , Vec < (N , CompactDirection) > , S > , edges : IndexMap < (N , N) , E , S > , ty : PhantomData < Ty > , }
    };
}

GraphMap!()