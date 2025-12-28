macro_rules! deps {
    () => {
        ModeratePathPowers!();
    };
}

macro_rules! ModeratePathCache {
    () => {
        deps!();
        # [doc = " Cached powers as a trait for a floating-point type."] pub (crate) trait ModeratePathCache { # [doc = " Get cached powers."] fn get_powers () -> & 'static ModeratePathPowers ; }
    };
}

ModeratePathCache!()