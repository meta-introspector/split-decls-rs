macro_rules! deps {
    () => {
        MatrixGraph!();
        MatrixError!();
    };
}

macro_rules! impl_985 {
    () => {
        deps!();
        impl fmt :: Display for MatrixError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { MatrixError :: NodeIxLimit => write ! (f , "The MatrixGraph is at the maximum number of nodes for its index") , MatrixError :: NodeMissed (i) => { write ! (f , "The node with index {i} is missing from the graph.") } } } }
    };
}

impl_985!();