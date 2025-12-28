macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < T : Arbitrary + Clone + PartialOrd > Arbitrary for RangeTo < T > { fn arbitrary (g : & mut Gen) -> RangeTo < T > { .. Arbitrary :: arbitrary (g) } fn shrink (& self) -> Box < dyn Iterator < Item = RangeTo < T > > > { Box :: new (self . end . clone () . shrink () . map (| end | .. end)) } }
    };
}

impl_61!()