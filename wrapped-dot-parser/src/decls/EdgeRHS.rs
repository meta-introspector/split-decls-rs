macro_rules! deps {
    () => {
        NodeID!();
    };
}

macro_rules! EdgeRHS {
    () => {
        deps!();
        # [doc = " The Right hand side of an edge description. This corresponds to the"] # [doc = " `EdgeRHS` non-terminal of the grammar."] # [doc = " Notice that the grammar allows multiple EdgeRHS in sequence, to chain edges:"] # [doc = " `A -> B -> C`."] # [doc = " Notice that, contrary to Ast::EdgeRHS, Ast::FlatGraph::EdgeRHS can not contain a subgraph."] # [derive (Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct EdgeRHS { # [doc = " The identifier of the destination of the edge."] pub to : NodeID , # [doc = " A possible chained RHS."] pub next : Option < Box < EdgeRHS > > , }
    };
}

EdgeRHS!();