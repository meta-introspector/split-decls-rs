macro_rules! deps {
    () => {
        FieldElement!();
    };
}

macro_rules! impl_420 {
    () => {
        deps!();
        impl Mul for FieldElement { type Output = Self ; fn mul (self , other : Self) -> Self { Self (barrett_reduce (self . 0 * other . 0)) } }
    };
}

impl_420!();