macro_rules! macro_95 {
    () => {
        trait_template ! { # [doc = " The graph’s `NodeId`s map to indices"] # [allow (clippy :: needless_arbitrary_self_type)] pub trait EdgeIndexable : GraphBase { @ section self # [doc = " Return an upper bound of the edge indices in the graph"] # [doc = " (suitable for the size of a bitmap)."] fn edge_bound (self : & Self) -> usize ; # [doc = " Convert `a` to an integer index."] # [track_caller] fn to_index (self : & Self , a : Self :: EdgeId) -> usize ; # [doc = " Convert `i` to an edge index. `i` must be a valid value in the graph."] # [track_caller] fn from_index (self : & Self , i : usize) -> Self :: EdgeId ; } }
    };
}

macro_95!()