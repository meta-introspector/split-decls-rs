macro_rules! deps {
    () => {
        ModeratePathPowers!();
        ExtendedFloat!();
        ModeratePathCache!();
    };
}

macro_rules! impl_397 {
    () => {
        deps!();
        impl ModeratePathCache for ExtendedFloat { # [inline] fn get_powers () -> & 'static ModeratePathPowers { cached_float80 :: get_powers () } }
    };
}

impl_397!();