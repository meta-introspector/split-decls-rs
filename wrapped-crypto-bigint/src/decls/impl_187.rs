macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        # [doc = " Any non-zero integer multiplied by another non-zero integer is definitionally non-zero."] impl < T > Mul < Self > for NonZero < T > where T : Mul < T , Output = T > , { type Output = Self ; fn mul (self , rhs : Self) -> Self { Self (self . 0 * rhs . 0) } }
    };
}

impl_187!()