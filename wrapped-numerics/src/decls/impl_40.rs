macro_rules! deps {
    () => {
        Matrix4x4!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl core :: ops :: Sub < & Matrix4x4 > for & Matrix4x4 { type Output = Matrix4x4 ; fn sub (self , rhs : & Matrix4x4) -> Matrix4x4 { self . impl_sub (rhs) } }
    };
}

impl_40!();