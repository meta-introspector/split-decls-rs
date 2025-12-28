macro_rules! deps {
    () => {
        Vec4!();
        Vec2!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < S3 , S4 , NI > Vec4 < u64 > for u64x4_sse2 < S3 , S4 , NI > where u64x2_sse2 < S3 , S4 , NI > : Copy + Vec2 < u64 > , { # [inline (always)] fn extract (self , i : u32) -> u64 { match i { 0 => self . 0 [0] . extract (0) , 1 => self . 0 [0] . extract (1) , 2 => self . 0 [1] . extract (0) , 3 => self . 0 [1] . extract (1) , _ => panic ! () , } } # [inline (always)] fn insert (mut self , w : u64 , i : u32) -> Self { match i { 0 => self . 0 [0] = self . 0 [0] . insert (w , 0) , 1 => self . 0 [0] = self . 0 [0] . insert (w , 1) , 2 => self . 0 [1] = self . 0 [1] . insert (w , 0) , 3 => self . 0 [1] = self . 0 [1] . insert (w , 1) , _ => panic ! () , } ; self } }
    };
}

impl_186!();