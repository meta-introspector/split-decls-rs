macro_rules! deps {
    () => {
        Vector3!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl core :: ops :: Div < f32 > for Vector3 { type Output = Self ; fn div (self , rhs : f32) -> Self { self . impl_div_f32 (rhs) } }
    };
}

impl_83!();