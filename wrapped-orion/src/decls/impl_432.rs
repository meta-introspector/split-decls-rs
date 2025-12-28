macro_rules! deps {
    () => {
        RingElement!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl Sub for RingElement { type Output = Self ; fn sub (self , other : Self) -> Self { let mut ret_sub = Self :: zero () ; sub_poly (& self . coefficients , & other . coefficients , & mut ret_sub . coefficients ,) ; ret_sub } }
    };
}

impl_432!()