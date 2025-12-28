macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < A : Arbitrary > Arbitrary for Option < A > { fn arbitrary (g : & mut Gen) -> Option < A > { if g . random () { None } else { Some (Arbitrary :: arbitrary (g)) } } fn shrink (& self) -> Box < dyn Iterator < Item = Option < A > > > { match * self { None => empty_shrinker () , Some (ref x) => { let chain = single_shrinker (None) . chain (x . shrink () . map (Some)) ; Box :: new (chain) } } } }
    };
}

impl_12!();