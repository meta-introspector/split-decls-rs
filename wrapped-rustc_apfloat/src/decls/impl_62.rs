macro_rules! deps {
    () => {
        DoubleFloat!();
        Float!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < F : Float > Neg for DoubleFloat < F > { type Output = Self ; fn neg (self) -> Self { if self . 1 . is_finite_non_zero () { DoubleFloat (- self . 0 , - self . 1) } else { DoubleFloat (- self . 0 , self . 1) } } }
    };
}

impl_62!();