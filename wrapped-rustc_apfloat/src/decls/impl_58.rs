macro_rules! deps {
    () => {
        Fallback!();
        ExpInt!();
        Float!();
        FallbackExtendedS!();
        Semantics!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < F : Float > ieee :: Semantics for FallbackExtendedS < F > { const BITS : usize = 0 ; const EXP_BITS : usize = 0 ; const PRECISION : usize = Fallback :: < F > :: PRECISION ; const MAX_EXP : ExpInt = F :: MAX_EXP as ExpInt ; const MIN_EXP : ExpInt = F :: MIN_EXP as ExpInt ; }
    };
}

impl_58!()