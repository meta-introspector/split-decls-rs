macro_rules! deps {
    () => {
        Port!();
    };
}

macro_rules! NodeID {
    () => {
        deps!();
        # [doc = " This structure corresponds to the `node_id` non-terminal of the grammar."] # [doc = " If contains the identifier of the node, and possibly a port."] # [derive (Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct NodeID { # [doc = " The identifier of the node."] pub id : String , # [doc = " The port of the node, if any."] pub port : Option < Port > , }
    };
}

NodeID!()