macro_rules! deps {
    () => {
        Vec4!();
    };
}

macro_rules! impl_340 {
    () => {
        deps!();
        impl Vec4 < u64 > for u64x4_generic { # [inline (always)] fn extract (self , i : u32) -> u64 { let d : [u64 ; 4] = self . to_lanes () ; d [i as usize] } # [inline (always)] fn insert (self , v : u64 , i : u32) -> Self { self . 0 [(i / 2) as usize] . insert (v , i % 2) ; self } }
    };
}

impl_340!();