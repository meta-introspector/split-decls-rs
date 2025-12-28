macro_rules! deps {
    () => {
        Rgb!();
    };
}

macro_rules! rgb_mul_f32 {
    () => {
        deps!();
        fn rgb_mul_f32 (lhs : & Rgb , rhs : & f32) -> Rgb { Rgb :: new ((lhs . r as f32 * rhs . clamp (0.0 , 1.0)) as u8 , (lhs . g as f32 * rhs . clamp (0.0 , 1.0)) as u8 , (lhs . b as f32 * rhs . clamp (0.0 , 1.0)) as u8 ,) }
    };
}

rgb_mul_f32!()