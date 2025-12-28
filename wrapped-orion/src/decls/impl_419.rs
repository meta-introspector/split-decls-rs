macro_rules! deps {
    () => {
        FieldElement!();
    };
}

macro_rules! impl_419 {
    () => {
        deps!();
        impl Sub for FieldElement { type Output = Self ; fn sub (self , other : Self) -> Self { let x : u32 = self . 0 . overflowing_sub (other . 0) . 0 . overflowing_add (KYBER_Q) . 0 ; Self (conditional_sub_u32 (x)) } }
    };
}

impl_419!();