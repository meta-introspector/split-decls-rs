macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < T : Arbitrary + Clone + PartialOrd > Arbitrary for RangeInclusive < T > { fn arbitrary (g : & mut Gen) -> RangeInclusive < T > { Arbitrary :: arbitrary (g) ..= Arbitrary :: arbitrary (g) } fn shrink (& self) -> Box < dyn Iterator < Item = RangeInclusive < T > > > { Box :: new ((self . start () . clone () , self . end () . clone ()) . shrink () . map (| (s , e) | s ..= e) ,) } }
    };
}

impl_59!();