macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < A : Arbitrary > Arbitrary for Box < A > { fn arbitrary (g : & mut Gen) -> Box < A > { Box :: new (A :: arbitrary (g)) } fn shrink (& self) -> Box < dyn Iterator < Item = Box < A > > > { Box :: new ((* * self) . shrink () . map (Box :: new)) } }
    };
}

impl_65!();