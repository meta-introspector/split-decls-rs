macro_rules! deps {
    () => {
        NeighborsDirected!();
        Direction!();
        Directed!();
        Undirected!();
    };
}

macro_rules! macro_71 {
    () => {
        deps!();
        trait_template ! { # [doc = " Access to the neighbors of each node, through incoming or outgoing edges."] # [doc = ""] # [doc = " Depending on the graph’s edge type, the neighbors of a given directionality"] # [doc = " are:"] # [doc = ""] # [doc = " - `Directed`, `Outgoing`: All targets of edges from `a`."] # [doc = " - `Directed`, `Incoming`: All sources of edges to `a`."] # [doc = " - `Undirected`: All other endpoints of edges connected to `a`."] pub trait IntoNeighborsDirected : IntoNeighbors { @ section type type NeighborsDirected : Iterator < Item = Self :: NodeId >; @ section self fn neighbors_directed (self , n : Self :: NodeId , d : Direction) -> Self :: NeighborsDirected ; } }
    };
}

macro_71!();