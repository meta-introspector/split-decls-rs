macro_rules! deps {
    () => {
        NodeIndex!();
        Graph!();
        IndexType!();
        EdgeIndex!();
    };
}

macro_rules! impl_735 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > visit :: GraphBase for Graph < N , E , Ty , Ix > where Ix : IndexType , { type NodeId = NodeIndex < Ix > ; type EdgeId = EdgeIndex < Ix > ; }
    };
}

impl_735!()