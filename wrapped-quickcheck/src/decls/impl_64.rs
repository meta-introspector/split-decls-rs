macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl Arbitrary for Duration { fn arbitrary (rng : & mut Gen) -> Self { let seconds = rng . random_range (0 .. rng . size () as u64) ; let nanoseconds = rng . random_range (0 .. 1_000_000) ; Duration :: new (seconds , nanoseconds) } fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { Box :: new ((self . as_secs () , self . subsec_nanos ()) . shrink () . map (| (secs , nanos) | Duration :: new (secs , nanos % 1_000_000)) ,) } }
    };
}

impl_64!()