macro_rules! deps {
    () => {
        Vector3!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl core :: ops :: Add < Vector3 > for & Vector3 { type Output = Vector3 ; fn add (self , rhs : Vector3) -> Vector3 { self . impl_add (& rhs) } }
    };
}

impl_73!();