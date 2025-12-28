macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl Arbitrary for SystemTime { fn arbitrary (rng : & mut Gen) -> Self { let after_epoch = bool :: arbitrary (rng) ; let duration = Duration :: arbitrary (rng) ; if after_epoch { UNIX_EPOCH + duration } else { UNIX_EPOCH - duration } } fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { let duration = match self . duration_since (UNIX_EPOCH) { Ok (duration) => duration , Err (e) => e . duration () , } ; Box :: new (duration . shrink () . flat_map (| d | vec ! [UNIX_EPOCH + d , UNIX_EPOCH - d]) ,) } }
    };
}

impl_67!();