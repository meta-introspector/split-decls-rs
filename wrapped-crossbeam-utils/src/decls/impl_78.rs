macro_rules! deps {
    () => {
        CachePadded!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < T > From < T > for CachePadded < T > { fn from (t : T) -> Self { Self :: new (t) } }
    };
}

impl_78!()