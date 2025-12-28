macro_rules! deps {
    () => {
        Row!();
        IndexType!();
        DefaultIx!();
    };
}

macro_rules! List {
    () => {
        deps!();
        # [doc = " An adjacency list with labeled edges."] # [doc = ""] # [doc = " Can be interpreted as a directed graph"] # [doc = " with unweighted nodes."] # [doc = ""] # [doc = " This is the most simple adjacency list you can imagine. [`Graph`](../graph/struct.Graph.html), in contrast,"] # [doc = " maintains both the list of successors and predecessors for each node,"] # [doc = " which is a different trade-off."] # [doc = ""] # [doc = " Allows parallel edges and self-loops."] # [doc = ""] # [doc = " This data structure is append-only (except for [`clear`](#method.clear)), so indices"] # [doc = " returned at some point for a given graph will stay valid with this same"] # [doc = " graph until it is dropped or [`clear`](#method.clear) is called."] # [doc = ""] # [doc = " Space consumption: **O(|E|)**."] # [derive (Clone , Default)] pub struct List < E , Ix = DefaultIx > where Ix : IndexType , { suc : Vec < Row < E , Ix > > , }
    };
}

List!();