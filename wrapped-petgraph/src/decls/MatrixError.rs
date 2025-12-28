macro_rules! deps {
    () => {
        MatrixGraph!();
    };
}

macro_rules! MatrixError {
    () => {
        deps!();
        # [doc = " The error type for fallible `MatrixGraph` operations."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum MatrixError { # [doc = " The `MatrixGraph` is at the maximum number of nodes for its index."] NodeIxLimit , # [doc = " The node with the specified index is missing from the graph."] NodeMissed (usize) , }
    };
}

MatrixError!()