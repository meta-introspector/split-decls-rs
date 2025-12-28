macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < T : Arbitrary + Ord > Arbitrary for BTreeSet < T > { fn arbitrary (g : & mut Gen) -> BTreeSet < T > { let vec : Vec < T > = Arbitrary :: arbitrary (g) ; vec . into_iter () . collect () } fn shrink (& self) -> Box < dyn Iterator < Item = BTreeSet < T > > > { let vec : Vec < T > = self . clone () . into_iter () . collect () ; Box :: new (vec . shrink () . map (| v | v . into_iter () . collect :: < BTreeSet < T > > ())) } }
    };
}

impl_24!()