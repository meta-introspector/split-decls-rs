macro_rules! deps {
    () => {
        GraphRef!();
        EdgeRef!();
        EdgeReferences!();
    };
}

macro_rules! macro_89 {
    () => {
        deps!();
        trait_template ! { # [doc = " Access to the sequence of the graph’s edges"] pub trait IntoEdgeReferences : Data + GraphRef { @ section type type EdgeRef : EdgeRef < NodeId = Self :: NodeId , EdgeId = Self :: EdgeId , Weight = Self :: EdgeWeight >; type EdgeReferences : Iterator < Item = Self :: EdgeRef >; @ section self fn edge_references (self) -> Self :: EdgeReferences ; } }
    };
}

macro_89!();