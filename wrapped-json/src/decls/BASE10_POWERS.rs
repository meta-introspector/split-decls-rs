macro_rules! deps {
    () => {
        ModeratePathPowers!();
        ExtendedFloatArray!();
    };
}

macro_rules! BASE10_POWERS {
    () => {
        deps!();
        const BASE10_POWERS : ModeratePathPowers = ModeratePathPowers { small : ExtendedFloatArray { mant : & BASE10_SMALL_MANTISSA , exp : & BASE10_SMALL_EXPONENT , } , large : ExtendedFloatArray { mant : & BASE10_LARGE_MANTISSA , exp : & BASE10_LARGE_EXPONENT , } , small_int : & BASE10_SMALL_INT_POWERS , step : BASE10_STEP , bias : BASE10_BIAS , } ;
    };
}

BASE10_POWERS!();