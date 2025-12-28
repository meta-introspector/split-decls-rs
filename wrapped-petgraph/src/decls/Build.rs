macro_rules! Build {
    () => {
        # [doc = " A graph that can be extended with further nodes and edges"] pub trait Build : Data + NodeCount { fn add_node (& mut self , weight : Self :: NodeWeight) -> Self :: NodeId ; # [doc = " Add a new edge. If parallel edges (duplicate) are not allowed and"] # [doc = " the edge already exists, return `None`."] # [doc = ""] # [doc = " Might panic if `a` or `b` are out of bounds."] # [track_caller] fn add_edge (& mut self , a : Self :: NodeId , b : Self :: NodeId , weight : Self :: EdgeWeight ,) -> Option < Self :: EdgeId > { Some (self . update_edge (a , b , weight)) } # [doc = " Add or update the edge from `a` to `b`. Return the id of the affected"] # [doc = " edge."] # [doc = ""] # [doc = " Might panic if `a` or `b` are out of bounds."] # [track_caller] fn update_edge (& mut self , a : Self :: NodeId , b : Self :: NodeId , weight : Self :: EdgeWeight ,) -> Self :: EdgeId ; }
    };
}

Build!();