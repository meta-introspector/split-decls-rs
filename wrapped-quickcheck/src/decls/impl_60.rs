macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < T : Arbitrary + Clone + PartialOrd > Arbitrary for RangeFrom < T > { fn arbitrary (g : & mut Gen) -> RangeFrom < T > { Arbitrary :: arbitrary (g) .. } fn shrink (& self) -> Box < dyn Iterator < Item = RangeFrom < T > > > { Box :: new (self . start . clone () . shrink () . map (| start | start ..)) } }
    };
}

impl_60!()