macro_rules! deps {
    () => {
        Vector3!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl core :: ops :: Div < f32 > for & Vector3 { type Output = Vector3 ; fn div (self , rhs : f32) -> Vector3 { self . impl_div_f32 (rhs) } }
    };
}

impl_84!();