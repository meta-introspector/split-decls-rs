macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < A : Arbitrary + Sync > Arbitrary for Arc < A > { fn arbitrary (g : & mut Gen) -> Arc < A > { Arc :: new (A :: arbitrary (g)) } fn shrink (& self) -> Box < dyn Iterator < Item = Arc < A > > > { Box :: new ((* * self) . shrink () . map (Arc :: new)) } }
    };
}

impl_66!()