macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < T : Arbitrary > Arbitrary for Wrapping < T > { fn arbitrary (g : & mut Gen) -> Wrapping < T > { Wrapping (T :: arbitrary (g)) } fn shrink (& self) -> Box < dyn Iterator < Item = Wrapping < T > > > { Box :: new (self . 0 . shrink () . map (Wrapping)) } }
    };
}

impl_56!()