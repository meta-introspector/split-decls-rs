macro_rules! deps {
    () => {
        Vector4!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl core :: ops :: Div < f32 > for & Vector4 { type Output = Vector4 ; fn div (self , rhs : f32) -> Vector4 { self . impl_div_f32 (rhs) } }
    };
}

impl_106!()