macro_rules! deps {
    () => {
        Matching!();
    };
}

macro_rules! impl_400 {
    () => {
        deps!();
        impl < G > Matching < G > where G : NodeCount , { # [doc = " Returns `true` if the matching is perfect."] # [doc = ""] # [doc = " A matching is"] # [doc = " [*perfect*](https://en.wikipedia.org/wiki/Matching_(graph_theory)#Definitions)"] # [doc = " if every node in the graph is incident to an edge from the matching."] pub fn is_perfect (& self) -> bool { let n_nodes = self . graph . node_count () ; n_nodes % 2 == 0 && self . n_edges == n_nodes / 2 } }
    };
}

impl_400!()