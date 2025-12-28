macro_rules! deps {
    () => {
        IndexType!();
        EdgeIndex!();
        GraphIndex!();
    };
}

macro_rules! impl_719 {
    () => {
        deps!();
        impl < Ix : IndexType > GraphIndex for EdgeIndex < Ix > { # [inline] # [doc (hidden)] fn index (& self) -> usize { EdgeIndex :: index (* self) } # [inline] # [doc (hidden)] fn is_node_index () -> bool { false } }
    };
}

impl_719!();