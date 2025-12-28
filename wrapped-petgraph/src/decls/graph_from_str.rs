macro_rules! deps {
    () => {
        Graph!();
        ParseFromDot!();
    };
}

macro_rules! graph_from_str {
    () => {
        deps!();
        # [macro_export] # [doc = " Statically imports a Graph from a valid DOT/Graphviz [&str]."] macro_rules ! graph_from_str { ($ s : tt) => { $ crate :: dot :: dot_parser :: ParseFromDot :: from_dot_graph (dot_parser_macros :: from_dot_string ! ($ s)) } ; }
    };
}

graph_from_str!();