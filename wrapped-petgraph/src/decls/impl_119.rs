macro_rules! deps {
    () => {
        NodeFiltered!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < G , F > GraphBase for NodeFiltered < G , F > where G : GraphBase , { type NodeId = G :: NodeId ; type EdgeId = G :: EdgeId ; }
    };
}

impl_119!()