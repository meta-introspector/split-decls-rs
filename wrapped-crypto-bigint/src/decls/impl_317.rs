macro_rules! deps {
    () => {
        MultiExponentiate!();
        MultiExponentiateBoundedExp!();
        Bounded!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl < T , Exponent , BasesAndExponents > MultiExponentiate < Exponent , BasesAndExponents > for T where T : MultiExponentiateBoundedExp < Exponent , BasesAndExponents > , Exponent : Bounded , BasesAndExponents : AsRef < [(Self , Exponent)] > + ? Sized , { fn multi_exponentiate (bases_and_exponents : & BasesAndExponents) -> Self { Self :: multi_exponentiate_bounded_exp (bases_and_exponents , Exponent :: BITS) } }
    };
}

impl_317!()