macro_rules! deps {
    () => {
        Unsigned!();
        Signed!();
        Int!();
        Uint!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < const LIMBS : usize > Signed for Int < LIMBS > { type Unsigned = Uint < LIMBS > ; fn abs_sign (& self) -> (Uint < LIMBS > , Choice) { let (abs , sign) = self . abs_sign () ; (abs , sign . into ()) } fn is_negative (& self) -> Choice { self . is_negative () . into () } fn is_positive (& self) -> Choice { self . is_positive () . into () } }
    };
}

impl_110!()