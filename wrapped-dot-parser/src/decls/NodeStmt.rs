macro_rules! deps {
    () => {
        NodeID!();
        AttrList!();
    };
}

macro_rules! NodeStmt {
    () => {
        deps!();
        # [doc = " This structure corresponds to the `node_stmt` non-terminal of the grammar."] # [doc = " It is basically a node identifier attached to some attributes."] # [derive (Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct NodeStmt < A > { # [doc = " The identifier of the node."] pub node : NodeID , # [doc = " The possible list of attributes."] pub attr : Option < AttrList < A > > , }
    };
}

NodeStmt!()