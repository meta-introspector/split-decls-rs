macro_rules! deps {
    () => {
        NodeID!();
        EdgeRHS!();
        AttrList!();
    };
}

macro_rules! EdgeStmt {
    () => {
        deps!();
        # [doc = " The description of an edge. This corresponds to the `edge_stmt` non-terminal"] # [doc = " of the grammar."] # [derive (Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct EdgeStmt < A > { # [doc = " The origin of the edge."] pub from : NodeID , # [doc = " The destination of the edge."] pub next : EdgeRHS , # [doc = " The attributes of the edge."] pub attr : Option < AttrList < A > > , }
    };
}

EdgeStmt!();