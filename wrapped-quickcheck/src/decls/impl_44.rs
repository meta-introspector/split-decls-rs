macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl Arbitrary for usize { fn arbitrary (g : & mut Gen) -> usize { match g . random_range (0 .. 10) { 0 => * g . choose (unsigned_problem_values ! (usize)) . unwrap () , _ => { # [cfg (target_pointer_width = "16")] { g . random :: < u16 > () as usize } # [cfg (target_pointer_width = "32")] { g . random :: < u32 > () as usize } # [cfg (target_pointer_width = "64")] { g . random :: < u64 > () as usize } } } } fn shrink (& self) -> Box < dyn Iterator < Item = usize > > { unsigned_shrinker ! (usize) ; shrinker :: UnsignedShrinker :: new (* self) } }
    };
}

impl_44!();