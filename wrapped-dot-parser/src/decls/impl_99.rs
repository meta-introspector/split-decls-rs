macro_rules! deps {
    () => {
        NodeStmt!();
        NodeSet!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < A , I > From < I > for NodeSet < A > where I : IntoIterator < Item = NodeStmt < A > > , { fn from (nodes : I) -> Self { let set : HashMap < _ , _ > = nodes . into_iter () . map (| node | (node . node . id . to_string () , node . into ())) . collect () ; NodeSet { set } } }
    };
}

impl_99!();