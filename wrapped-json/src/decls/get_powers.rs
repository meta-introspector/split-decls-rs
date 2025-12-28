macro_rules! deps {
    () => {
        ModeratePathPowers!();
    };
}

macro_rules! get_powers {
    () => {
        deps!();
        # [doc = " Get powers from base."] pub (crate) fn get_powers () -> & 'static ModeratePathPowers { & BASE10_POWERS }
    };
}

get_powers!();