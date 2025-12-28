macro_rules! deps {
    () => {
        Vec4!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < W : Copy > Vec4 < W > for x4 < W > { # [inline (always)] fn extract (self , i : u32) -> W { self . 0 [i as usize] } # [inline (always)] fn insert (mut self , w : W , i : u32) -> Self { self . 0 [i as usize] = w ; self } }
    };
}

impl_56!()