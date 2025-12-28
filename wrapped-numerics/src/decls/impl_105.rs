macro_rules! deps {
    () => {
        Vector4!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl core :: ops :: Div < f32 > for Vector4 { type Output = Self ; fn div (self , rhs : f32) -> Self { self . impl_div_f32 (rhs) } }
    };
}

impl_105!()