macro_rules! deps {
    () => {
        EdgeType!();
        Nullable!();
        Edges!();
    };
}

macro_rules! Neighbors {
    () => {
        deps!();
        # [doc = " Iterator over the neighbors of a node."] # [doc = ""] # [doc = " Iterator element type is `NodeIndex<Ix>`."] # [doc = ""] # [doc = " Created with [`.neighbors()`][1], [`.neighbors_directed()`][2]."] # [doc = ""] # [doc = " [1]: struct.MatrixGraph.html#method.neighbors"] # [doc = " [2]: struct.MatrixGraph.html#method.neighbors_directed"] # [derive (Debug , Clone)] pub struct Neighbors < 'a , Ty : EdgeType , Null : 'a + Nullable , Ix > (Edges < 'a , Ty , Null , Ix >) ;
    };
}

Neighbors!()