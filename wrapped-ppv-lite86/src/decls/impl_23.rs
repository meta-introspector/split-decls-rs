macro_rules! deps {
    () => {
        UnsafeFrom!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < W , G > UnsafeFrom < [W ; 2] > for x2 < W , G > { # [inline (always)] unsafe fn unsafe_from (xs : [W ; 2]) -> Self { x2 :: new (xs) } }
    };
}

impl_23!();