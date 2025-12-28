macro_rules! impl_1 {
    () => {
        impl < W , G > x2 < W , G > { # [inline (always)] pub fn new (xs : [W ; 2]) -> Self { x2 (xs , PhantomData) } }
    };
}

impl_1!()