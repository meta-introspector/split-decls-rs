macro_rules! deps {
    () => {
        Vector4!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl core :: ops :: Div < Vector4 > for & Vector4 { type Output = Vector4 ; fn div (self , rhs : Vector4) -> Vector4 { self . impl_div (& rhs) } }
    };
}

impl_103!()