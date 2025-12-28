macro_rules! deps {
    () => {
        RandomState!();
        SharedSeed!();
        FixedState!();
    };
}

macro_rules! FIXED_GLOBAL_SEED {
    () => {
        deps!();
        # [doc = " Used for FixedState, and RandomState if atomics for dynamic init are unavailable."] const FIXED_GLOBAL_SEED : SharedSeed = SharedSeed { seeds : [ARBITRARY6 , ARBITRARY7 , ARBITRARY8 , ARBITRARY9 , ARBITRARY10 , ARBITRARY11 ,] , } ;
    };
}

FIXED_GLOBAL_SEED!()