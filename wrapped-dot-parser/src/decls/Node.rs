macro_rules! deps {
    () => {
        Port!();
        AList!();
    };
}

macro_rules! Node {
    () => {
        deps!();
        # [doc = " A single node of the graph."] # [derive (Debug , Clone)] pub struct Node < A > { # [doc = " The identifier of the node."] pub id : String , # [doc = " The port of the node."] pub port : Option < Port > , # [doc = " The attributes that apply to this node."] pub attr : AList < A > , }
    };
}

Node!();