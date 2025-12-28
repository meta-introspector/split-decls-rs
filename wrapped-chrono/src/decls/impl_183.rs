macro_rules! deps {
    () => {
        DateTime!();
        Offset!();
        TimeZone!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        # [cfg (feature = "defmt")] impl < Tz : TimeZone > defmt :: Format for DateTime < Tz > where Tz :: Offset : defmt :: Format , { fn format (& self , fmt : defmt :: Formatter) { defmt :: write ! (fmt , "{}{}" , self . overflowing_naive_local () , self . offset) ; } }
    };
}

impl_183!();