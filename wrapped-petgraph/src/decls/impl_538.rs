macro_rules! deps {
    () => {
        IndexType!();
        Csr!();
        Neighbors!();
        EdgeType!();
        Undirected!();
        Directed!();
    };
}

macro_rules! impl_538 {
    () => {
        deps!();
        impl < 'a , N , E , Ty , Ix > IntoNeighbors for & 'a Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Neighbors = Neighbors < 'a , Ix > ; # [doc = " Return an iterator of all neighbors of `a`."] # [doc = ""] # [doc = " - `Directed`: Targets of outgoing edges from `a`."] # [doc = " - `Undirected`: Opposing endpoints of all edges connected to `a`."] # [doc = ""] # [doc = " **Panics** if the node `a` does not exist.<br>"] # [doc = " Iterator element type is `NodeIndex<Ix>`."] # [track_caller] fn neighbors (self , a : Self :: NodeId) -> Self :: Neighbors { Neighbors { iter : self . neighbors_slice (a) . iter () , } } }
    };
}

impl_538!();