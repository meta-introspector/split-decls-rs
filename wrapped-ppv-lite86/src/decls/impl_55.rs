macro_rules! deps {
    () => {
        UnsafeFrom!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < W > UnsafeFrom < [W ; 4] > for x4 < W > { # [inline (always)] unsafe fn unsafe_from (xs : [W ; 4]) -> Self { x4 (xs) } }
    };
}

impl_55!()