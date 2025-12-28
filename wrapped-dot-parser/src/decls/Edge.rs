macro_rules! deps {
    () => {
        AList!();
    };
}

macro_rules! Edge {
    () => {
        deps!();
        # [doc = " A single edge of the graph."] # [derive (Debug , Clone , Eq , PartialEq)] pub struct Edge < A > { # [doc = " The name of the origin of the edge."] pub from : String , # [doc = " The name of the destination of the edge."] pub to : String , # [doc = " A list of attributes that apply to this specific edge."] pub attr : AList < A > , }
    };
}

Edge!()