macro_rules! deps {
    () => {
        EdgeIndex!();
        NodeIndex!();
        StableGraph!();
        IndexType!();
    };
}

macro_rules! impl_848 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > visit :: GraphBase for StableGraph < N , E , Ty , Ix > where Ix : IndexType , { type NodeId = NodeIndex < Ix > ; type EdgeId = EdgeIndex < Ix > ; }
    };
}

impl_848!()