macro_rules! deps {
    () => {
        ExpInt!();
        Float!();
        Semantics!();
        FallbackS!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < F : Float > ieee :: Semantics for FallbackS < F > { const BITS : usize = 0 ; const EXP_BITS : usize = 0 ; const PRECISION : usize = F :: PRECISION * 2 ; const MAX_EXP : ExpInt = F :: MAX_EXP as ExpInt ; const MIN_EXP : ExpInt = F :: MIN_EXP as ExpInt + F :: PRECISION as ExpInt ; }
    };
}

impl_55!();