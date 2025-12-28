macro_rules! deps {
    () => {
        Vector2!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl core :: ops :: Div < f32 > for & Vector2 { type Output = Vector2 ; fn div (self , rhs : f32) -> Vector2 { self . impl_div_f32 (rhs) } }
    };
}

impl_62!()