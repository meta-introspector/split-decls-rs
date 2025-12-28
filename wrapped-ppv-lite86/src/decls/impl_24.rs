macro_rules! deps {
    () => {
        Vec2!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < W : Copy , G > Vec2 < W > for x2 < W , G > { # [inline (always)] fn extract (self , i : u32) -> W { self . 0 [i as usize] } # [inline (always)] fn insert (mut self , w : W , i : u32) -> Self { self . 0 [i as usize] = w ; self } }
    };
}

impl_24!()