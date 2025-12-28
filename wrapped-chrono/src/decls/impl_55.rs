macro_rules! deps {
    () => {
        TimeZone!();
        Date!();
        Offset!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        # [cfg (feature = "defmt")] impl < Tz : TimeZone > defmt :: Format for Date < Tz > where Tz :: Offset : defmt :: Format , { fn format (& self , fmt : defmt :: Formatter) { defmt :: write ! (fmt , "{}{}" , self . naive_local () , self . offset) ; } }
    };
}

impl_55!()