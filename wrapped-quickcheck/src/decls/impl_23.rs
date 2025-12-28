macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < K : Arbitrary + Eq + Hash , V : Arbitrary , S : BuildHasher + Default + Clone + 'static , > Arbitrary for HashMap < K , V , S > { fn arbitrary (g : & mut Gen) -> Self { let vec : Vec < (K , V) > = Arbitrary :: arbitrary (g) ; vec . into_iter () . collect () } fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { let vec : Vec < (K , V) > = self . clone () . into_iter () . collect () ; Box :: new (vec . shrink () . map (| v | v . into_iter () . collect :: < Self > ())) } }
    };
}

impl_23!()