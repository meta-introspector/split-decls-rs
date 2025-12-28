macro_rules! deps {
    () => {
        NodeIndex!();
        IndexType!();
    };
}

macro_rules! impl_657 {
    () => {
        deps!();
        impl < Ix : IndexType > From < Ix > for NodeIndex < Ix > { fn from (ix : Ix) -> Self { NodeIndex (ix) } }
    };
}

impl_657!()