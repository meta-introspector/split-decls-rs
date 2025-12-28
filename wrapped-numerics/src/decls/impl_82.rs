macro_rules! deps {
    () => {
        Vector3!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl core :: ops :: Div < & Vector3 > for & Vector3 { type Output = Vector3 ; fn div (self , rhs : & Vector3) -> Vector3 { self . impl_div (rhs) } }
    };
}

impl_82!();