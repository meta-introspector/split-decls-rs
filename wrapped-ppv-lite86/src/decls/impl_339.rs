macro_rules! deps {
    () => {
        Vec4!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl Vec4 < u32 > for u32x4_generic { # [inline (always)] fn extract (self , i : u32) -> u32 { self . 0 [i as usize] } # [inline (always)] fn insert (mut self , v : u32 , i : u32) -> Self { self . 0 [i as usize] = v ; self } }
    };
}

impl_339!();