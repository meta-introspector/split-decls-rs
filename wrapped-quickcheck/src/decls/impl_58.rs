macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < T : Arbitrary + Clone + PartialOrd > Arbitrary for Range < T > { fn arbitrary (g : & mut Gen) -> Range < T > { Arbitrary :: arbitrary (g) .. Arbitrary :: arbitrary (g) } fn shrink (& self) -> Box < dyn Iterator < Item = Range < T > > > { Box :: new ((self . start . clone () , self . end . clone ()) . shrink () . map (| (s , e) | s .. e) ,) } }
    };
}

impl_58!();