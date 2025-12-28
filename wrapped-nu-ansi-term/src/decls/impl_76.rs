macro_rules! deps {
    () => {
        Rgb!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl From < (f32 , f32 , f32) > for Rgb { fn from ((r , g , b) : (f32 , f32 , f32)) -> Self { Self :: from_f32 (r , g , b) } }
    };
}

impl_76!();