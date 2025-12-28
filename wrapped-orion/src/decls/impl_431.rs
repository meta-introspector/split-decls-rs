macro_rules! deps {
    () => {
        RingElement!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        impl Add for RingElement { type Output = Self ; fn add (self , other : Self) -> Self { let mut ret_add = Self :: zero () ; add_poly (& self . coefficients , & other . coefficients , & mut ret_add . coefficients ,) ; ret_add } }
    };
}

impl_431!();