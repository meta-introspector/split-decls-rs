macro_rules! deps {
    () => {
        Vector4!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl core :: ops :: Add < Vector4 > for & Vector4 { type Output = Vector4 ; fn add (self , rhs : Vector4) -> Vector4 { self . impl_add (& rhs) } }
    };
}

impl_95!()