macro_rules! deps {
    () => {
        GraphRef!();
        Undirected!();
        Directed!();
        Neighbors!();
    };
}

macro_rules! macro_69 {
    () => {
        deps!();
        trait_template ! { # [doc = " Access to the neighbors of each node"] # [doc = ""] # [doc = " The neighbors are, depending on the graph’s edge type:"] # [doc = ""] # [doc = " - `Directed`: All targets of edges from `a`."] # [doc = " - `Undirected`: All other endpoints of edges connected to `a`."] pub trait IntoNeighbors : GraphRef { @ section type type Neighbors : Iterator < Item = Self :: NodeId >; @ section self # [doc = " Return an iterator of the neighbors of node `a`."] fn neighbors (self , a : Self :: NodeId) -> Self :: Neighbors ; } }
    };
}

macro_69!();