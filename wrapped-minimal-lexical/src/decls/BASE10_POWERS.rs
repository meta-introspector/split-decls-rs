macro_rules! deps {
    () => {
        BellerophonPowers!();
    };
}

macro_rules! BASE10_POWERS {
    () => {
        deps!();
        pub const BASE10_POWERS : BellerophonPowers = BellerophonPowers { small : & BASE10_SMALL_MANTISSA , large : & BASE10_LARGE_MANTISSA , small_int : & BASE10_SMALL_INT_POWERS , step : BASE10_STEP , bias : BASE10_BIAS , log2 : BASE10_LOG2_MULT , log2_shift : BASE10_LOG2_SHIFT , } ;
    };
}

BASE10_POWERS!();