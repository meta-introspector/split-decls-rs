macro_rules! deps {
    () => {
        UnboundColumnFamily!();
        BoundColumnFamily!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl UnboundColumnFamily { pub (crate) fn bound_column_family < 'a > (self : Arc < Self >) -> Arc < BoundColumnFamily < 'a > > { unsafe { Arc :: from_raw (Arc :: into_raw (self) . cast ()) } } }
    };
}

impl_46!()