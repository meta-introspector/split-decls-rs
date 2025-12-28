macro_rules! deps {
    () => {
        Vector2!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl core :: ops :: Div < f32 > for Vector2 { type Output = Self ; fn div (self , rhs : f32) -> Self { self . impl_div_f32 (rhs) } }
    };
}

impl_61!()