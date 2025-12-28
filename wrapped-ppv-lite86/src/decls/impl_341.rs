macro_rules! deps {
    () => {
        Vec2!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        impl Vec2 < u64 > for u64x2_generic { # [inline (always)] fn extract (self , i : u32) -> u64 { self . 0 [i as usize] } # [inline (always)] fn insert (mut self , v : u64 , i : u32) -> Self { self . 0 [i as usize] = v ; self } }
    };
}

impl_341!();