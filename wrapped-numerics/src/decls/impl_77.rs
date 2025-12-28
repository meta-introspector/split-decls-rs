macro_rules! deps {
    () => {
        Vector3!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl core :: ops :: Sub < Vector3 > for & Vector3 { type Output = Vector3 ; fn sub (self , rhs : Vector3) -> Vector3 { self . impl_sub (& rhs) } }
    };
}

impl_77!();