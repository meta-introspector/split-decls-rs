macro_rules! deps {
    () => {
        IndexType!();
        EdgeIndex!();
    };
}

macro_rules! impl_663 {
    () => {
        deps!();
        impl < Ix : IndexType > From < Ix > for EdgeIndex < Ix > { fn from (ix : Ix) -> Self { EdgeIndex (ix) } }
    };
}

impl_663!();