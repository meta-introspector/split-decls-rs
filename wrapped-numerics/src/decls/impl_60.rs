macro_rules! deps {
    () => {
        Vector2!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl core :: ops :: Div < & Vector2 > for & Vector2 { type Output = Vector2 ; fn div (self , rhs : & Vector2) -> Vector2 { self . impl_div (rhs) } }
    };
}

impl_60!();