macro_rules! deps {
    () => {
        Vector4!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl core :: ops :: Sub < Vector4 > for & Vector4 { type Output = Vector4 ; fn sub (self , rhs : Vector4) -> Vector4 { self . impl_sub (& rhs) } }
    };
}

impl_99!()