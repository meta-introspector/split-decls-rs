macro_rules! deps {
    () => {
        Vector2!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl core :: ops :: Sub < Vector2 > for & Vector2 { type Output = Vector2 ; fn sub (self , rhs : Vector2) -> Vector2 { self . impl_sub (& rhs) } }
    };
}

impl_55!()