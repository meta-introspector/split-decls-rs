macro_rules! deps {
    () => {
        Matrix4x4!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl core :: ops :: Add < Matrix4x4 > for & Matrix4x4 { type Output = Matrix4x4 ; fn add (self , rhs : Matrix4x4) -> Matrix4x4 { self . impl_add (& rhs) } }
    };
}

impl_35!()