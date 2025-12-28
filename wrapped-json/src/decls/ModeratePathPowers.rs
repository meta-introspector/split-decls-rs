macro_rules! deps {
    () => {
        ExtendedFloatArray!();
    };
}

macro_rules! ModeratePathPowers {
    () => {
        deps!();
        # [doc = " Precalculated powers of base N for the moderate path."] # [doc (hidden)] pub (crate) struct ModeratePathPowers { pub small : ExtendedFloatArray , pub large : ExtendedFloatArray , # [doc = " Pre-calculated small powers as 64-bit integers"] pub small_int : & 'static [u64] , pub step : i32 , pub bias : i32 , }
    };
}

ModeratePathPowers!();