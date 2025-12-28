macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
        VecShrinker!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < A : Arbitrary > Arbitrary for Vec < A > { fn arbitrary (g : & mut Gen) -> Vec < A > { let size = { let s = g . size () ; g . random_range (0 .. s) } ; (0 .. size) . map (| _ | A :: arbitrary (g)) . collect () } fn shrink (& self) -> Box < dyn Iterator < Item = Vec < A > > > { VecShrinker :: new (self . clone ()) } }
    };
}

impl_18!()