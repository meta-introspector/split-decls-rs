macro_rules! deps {
    () => {
        IndexType!();
        GraphIndex!();
        NodeIndex!();
    };
}

macro_rules! impl_718 {
    () => {
        deps!();
        impl < Ix : IndexType > GraphIndex for NodeIndex < Ix > { # [inline] # [doc (hidden)] fn index (& self) -> usize { NodeIndex :: index (* self) } # [inline] # [doc (hidden)] fn is_node_index () -> bool { true } }
    };
}

impl_718!();