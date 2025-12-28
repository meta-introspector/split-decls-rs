macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < T : Arbitrary + Clone + PartialOrd > Arbitrary for RangeToInclusive < T > { fn arbitrary (g : & mut Gen) -> RangeToInclusive < T > { ..= Arbitrary :: arbitrary (g) } fn shrink (& self) -> Box < dyn Iterator < Item = RangeToInclusive < T > > > { Box :: new (self . end . clone () . shrink () . map (| end | ..= end)) } }
    };
}

impl_62!()