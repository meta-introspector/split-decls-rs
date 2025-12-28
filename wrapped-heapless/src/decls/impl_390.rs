macro_rules! deps {
    () => {
        VecInner!();
        LenType!();
    };
}

macro_rules! impl_390 {
    () => {
        deps!();
        impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > defmt :: Format for VecInner < T , LenT , S > where T : defmt :: Format , { fn format (& self , fmt : Formatter < '_ >) { defmt :: write ! (fmt , "{=[?]}" , self . as_slice ()) ; } }
    };
}

impl_390!();