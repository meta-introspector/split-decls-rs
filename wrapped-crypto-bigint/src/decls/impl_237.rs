macro_rules! deps {
    () => {
        Odd!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        # [doc = " Any odd integer multiplied by another odd integer is definitionally odd."] impl < T > Mul < Self > for Odd < T > where T : Mul < T , Output = T > , { type Output = Self ; fn mul (self , rhs : Self) -> Self { Self (self . 0 * rhs . 0) } }
    };
}

impl_237!()