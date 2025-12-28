macro_rules! deps {
    () => {
        Position!();
        ParentEdge!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl ParentEdge { pub fn from_raw (raw : u32) -> ParentEdge { if raw == NO_PARENT { return ParentEdge :: None ; } if raw & EXTENDED_EDGES_MASK != 0 { ParentEdge :: ExtraEdgeIndex (raw & ! EXTENDED_EDGES_MASK) } else { ParentEdge :: GraphPosition (Position (raw)) } } }
    };
}

impl_23!()