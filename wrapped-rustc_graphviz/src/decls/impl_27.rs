macro_rules! deps {
    () => {
        Edges!();
        Edge!();
        LabelledGraph!();
        Node!();
        GraphWalk!();
        Nodes!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < 'a > GraphWalk < 'a > for LabelledGraph { type Node = Node ; type Edge = & 'a Edge ; fn nodes (& 'a self) -> Nodes < 'a , Node > { (0 .. self . node_labels . len ()) . collect () } fn edges (& 'a self) -> Edges < 'a , & 'a Edge > { self . edges . iter () . collect () } fn source (& 'a self , edge : & & 'a Edge) -> Node { edge . from } fn target (& 'a self , edge : & & 'a Edge) -> Node { edge . to } }
    };
}

impl_27!();