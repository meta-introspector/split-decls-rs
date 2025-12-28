macro_rules! deps {
    () => {
        Bounded!();
        Pow!();
        PowBoundedExp!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl < T : PowBoundedExp < Exponent > , Exponent : Bounded > Pow < Exponent > for T { fn pow (& self , exponent : & Exponent) -> Self { self . pow_bounded_exp (exponent , Exponent :: BITS) } }
    };
}

impl_314!()