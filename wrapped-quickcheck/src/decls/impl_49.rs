macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl Arbitrary for isize { fn arbitrary (g : & mut Gen) -> isize { match g . random_range (0 .. 10) { 0 => * g . choose (signed_problem_values ! (isize)) . unwrap () , _ => { # [cfg (target_pointer_width = "16")] { g . random :: < i16 > () as isize } # [cfg (target_pointer_width = "32")] { g . random :: < i32 > () as isize } # [cfg (target_pointer_width = "64")] { g . random :: < i64 > () as isize } } } } fn shrink (& self) -> Box < dyn Iterator < Item = isize > > { signed_shrinker ! (isize) ; shrinker :: SignedShrinker :: new (* self) } }
    };
}

impl_49!()