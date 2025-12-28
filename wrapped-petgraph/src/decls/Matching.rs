macro_rules! Matching {
    () => {
        # [doc = " Computed"] # [doc = " [*matching*](https://en.wikipedia.org/wiki/Matching_(graph_theory)#Definitions)"] # [doc = " of the graph."] pub struct Matching < G : GraphBase > { graph : G , mate : Vec < Option < G :: NodeId > > , n_edges : usize , }
    };
}

Matching!();