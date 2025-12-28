macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < T : Arbitrary > Arbitrary for Bound < T > { fn arbitrary (g : & mut Gen) -> Bound < T > { match g . random_range (0 .. 3) { 0 => Bound :: Included (T :: arbitrary (g)) , 1 => Bound :: Excluded (T :: arbitrary (g)) , _ => Bound :: Unbounded , } } fn shrink (& self) -> Box < dyn Iterator < Item = Bound < T > > > { match * self { Bound :: Included (ref x) => { Box :: new (x . shrink () . map (Bound :: Included)) } Bound :: Excluded (ref x) => { Box :: new (x . shrink () . map (Bound :: Excluded)) } Bound :: Unbounded => empty_shrinker () , } } }
    };
}

impl_57!()