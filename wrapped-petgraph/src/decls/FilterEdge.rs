macro_rules! deps {
    () => {
        Edge!();
    };
}

macro_rules! FilterEdge {
    () => {
        deps!();
        # [doc = " A graph filter for edges"] pub trait FilterEdge < Edge > { # [doc = " Return true to have the edge be part of the graph"] fn include_edge (& self , edge : Edge) -> bool ; }
    };
}

FilterEdge!()