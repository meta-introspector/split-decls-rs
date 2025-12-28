macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Arbitrary for bool { fn arbitrary (g : & mut Gen) -> bool { g . random () } fn shrink (& self) -> Box < dyn Iterator < Item = bool > > { if * self { single_shrinker (false) } else { empty_shrinker () } } }
    };
}

impl_11!();