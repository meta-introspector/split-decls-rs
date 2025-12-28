macro_rules! deps {
    () => {
        Node!();
        Graph!();
        StableGraph!();
    };
}

macro_rules! GraphError {
    () => {
        deps!();
        # [doc = " The error type for fallible `Graph` & `StableGraph` operations."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum GraphError { # [doc = " The Graph is at the maximum number of nodes for its index."] NodeIxLimit , # [doc = " The Graph is at the maximum number of edges for its index."] EdgeIxLimit , # [doc = " The node with the specified index is missing from the graph."] NodeMissed (usize) , # [doc = " Node indices out of bounds."] NodeOutBounds , }
    };
}

GraphError!();