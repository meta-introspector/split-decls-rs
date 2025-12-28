macro_rules! deps {
    () => {
        Edge!();
        EdgeType!();
    };
}

macro_rules! macro_91 {
    () => {
        deps!();
        trait_template ! { # [doc = " Edge kind property (directed or undirected edges)"] pub trait GraphProp : GraphBase { @ section type # [doc = " The kind of edges in the graph."] type EdgeType : EdgeType ; @ section nodelegate fn is_directed (& self) -> bool { < Self :: EdgeType >:: is_directed () } } }
    };
}

macro_91!()