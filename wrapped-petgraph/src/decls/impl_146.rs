macro_rules! deps {
    () => {
        EdgeFiltered!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < G , F > GraphBase for EdgeFiltered < G , F > where G : GraphBase , { type NodeId = G :: NodeId ; type EdgeId = G :: EdgeId ; }
    };
}

impl_146!();