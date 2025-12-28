macro_rules! deps {
    () => {
        FieldElement!();
    };
}

macro_rules! impl_418 {
    () => {
        deps!();
        impl Add for FieldElement { type Output = Self ; fn add (self , other : Self) -> Self { let x : u32 = self . 0 + other . 0 ; Self (conditional_sub_u32 (x)) } }
    };
}

impl_418!();