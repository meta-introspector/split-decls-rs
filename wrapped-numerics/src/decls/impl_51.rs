macro_rules! deps {
    () => {
        Vector2!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl core :: ops :: Add < Vector2 > for & Vector2 { type Output = Vector2 ; fn add (self , rhs : Vector2) -> Vector2 { self . impl_add (& rhs) } }
    };
}

impl_51!();