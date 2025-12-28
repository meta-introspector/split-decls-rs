macro_rules! deps {
    () => {
        RootArcs!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl From < RootArcs > for u8 { fn from (root_arcs : RootArcs) -> u8 { root_arcs . 0 } }
    };
}

impl_15!();