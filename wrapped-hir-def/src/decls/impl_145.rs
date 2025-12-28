macro_rules! deps {
    () => {
        RawVisibilityId!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl RawVisibilityId { const PUB : Self = RawVisibilityId (u32 :: MAX) ; const PRIV_IMPLICIT : Self = RawVisibilityId (u32 :: MAX - 1) ; const PRIV_EXPLICIT : Self = RawVisibilityId (u32 :: MAX - 2) ; const PUB_CRATE : Self = RawVisibilityId (u32 :: MAX - 3) ; }
    };
}

impl_145!()