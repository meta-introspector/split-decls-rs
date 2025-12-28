macro_rules! deps {
    () => {
        Reversed!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < G : GraphBase > GraphBase for Reversed < G > { type NodeId = G :: NodeId ; type EdgeId = G :: EdgeId ; }
    };
}

impl_169!();