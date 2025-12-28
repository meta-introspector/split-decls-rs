macro_rules! deps {
    () => {
        LenType!();
        StringInner!();
    };
}

macro_rules! impl_391 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > defmt :: Format for StringInner < LenT , S > { fn format (& self , fmt : Formatter < '_ >) { defmt :: write ! (fmt , "{=str}" , self . as_str ()) ; } }
    };
}

impl_391!()