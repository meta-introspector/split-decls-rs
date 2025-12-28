macro_rules! deps {
    () => {
        ExtendedFloat!();
        ModeratePathCache!();
        ModeratePathPowers!();
    };
}

macro_rules! impl_397 {
    () => {
        deps!();
        impl ModeratePathCache for ExtendedFloat { # [inline] fn get_powers () -> & 'static ModeratePathPowers { cached_float80 :: get_powers () } }
    };
}

impl_397!()