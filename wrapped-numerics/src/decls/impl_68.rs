macro_rules! deps {
    () => {
        Vector2!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl core :: ops :: Mul < f32 > for & Vector2 { type Output = Vector2 ; fn mul (self , rhs : f32) -> Vector2 { self . impl_mul_f32 (rhs) } }
    };
}

impl_68!();