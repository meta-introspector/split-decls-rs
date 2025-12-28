macro_rules! deps {
    () => {
        IndexType!();
        WalkNeighbors!();
    };
}

macro_rules! impl_840 {
    () => {
        deps!();
        impl < Ix : IndexType > Clone for WalkNeighbors < Ix > { clone_fields ! (WalkNeighbors , inner) ; }
    };
}

impl_840!()