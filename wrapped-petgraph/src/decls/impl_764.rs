macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! impl_764 {
    () => {
        deps!();
        impl < G > GraphBase for Frozen < '_ , G > where G : GraphBase , { type NodeId = G :: NodeId ; type EdgeId = G :: EdgeId ; }
    };
}

impl_764!();