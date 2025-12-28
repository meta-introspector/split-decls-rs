macro_rules! deps {
    () => {
        List!();
        IndexType!();
        EdgeIndex!();
        NodeIndex!();
    };
}

macro_rules! impl_300 {
    () => {
        deps!();
        impl < E , Ix > visit :: GraphBase for List < E , Ix > where Ix : IndexType , { type NodeId = NodeIndex < Ix > ; type EdgeId = EdgeIndex < Ix > ; }
    };
}

impl_300!();